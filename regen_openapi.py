#!/usr/bin/env python3
"""
AFINSP SDK code generator orchestration script.

This script orchestrates the generation of SDK code from OpenAPI specs using
the openapi-codegen tool running in Docker containers.

Usage:
    ./regen_openapi.py

Environment variables:
    DEBUG=yes    Enable verbose output
    IMAGE=...    Override the Docker image to use
"""
# this file is @generated (but manually maintained - this comment prevents orphan detection)

import json
import os
import io
import tarfile
import random
import shutil
import string
import subprocess
import time
import multiprocessing
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

try:
    import tomllib
except ImportError:
    print("Python 3.11 or greater is required to run the codegen")
    exit(1)

# Default image - can be overridden with IMAGE env var
# For local development, build with: docker build -t afinsp-codegen codegen/
OPENAPI_CODEGEN_IMAGE = os.getenv("IMAGE", "ghcr.io/afinsp/openapi-codegen:latest")
DEBUG = os.getenv("DEBUG") is not None
GREEN = "\033[92m"
BLUE = "\033[94m"
CYAN = "\033[96m"
YELLOW = "\033[93m"
ENDC = "\033[0m"


class ExitException(Exception):
    """Exception to signal exit from a thread."""
    def __init__(self, code: int):
        super().__init__("")
        self.code = code


def get_docker_binary() -> str:
    """Find docker or podman binary."""
    # Prefer docker over podman
    docker_binary = shutil.which("docker")
    if docker_binary is None:
        docker_binary = shutil.which("podman")
    if docker_binary is None:
        print("Please install docker or podman to run the codegen")
        exit(1)
    return docker_binary


def docker_container_rm(prefix, container_id):
    cmd = [get_docker_binary(), "container", "rm", container_id]
    result = run_cmd(prefix, cmd)
    return result.stdout.decode("utf-8")


def docker_container_logs(prefix, container_id):
    cmd = [get_docker_binary(), "container", "logs", container_id]
    result = run_cmd(prefix, cmd, dont_dbg=True)
    return f"{result.stdout.decode('utf-8')}\n{result.stderr.decode('utf-8')}".strip()


def docker_container_wait(prefix, container_id) -> int:
    cmd = [get_docker_binary(), "container", "wait", container_id]
    result = run_cmd(prefix, cmd)
    return int(result.stdout.decode("utf-8"))


def docker_container_cp(prefix, container_id, task):
    cmd = [
        get_docker_binary(),
        "container",
        "cp",
        f"{container_id}:/app/{task['output_dir']}/.",
        f"{task['output_dir']}/",
    ]
    run_cmd(prefix, cmd)


def get_generated_paths(prefix, container_id, task) -> list[str]:
    cmd = [
        get_docker_binary(),
        "container",
        "cp",
        f"{container_id}:/app/.generated_paths.json",
        "-",
    ]
    result = run_cmd(prefix, cmd)
    with tarfile.open(fileobj=io.BytesIO(result.stdout), mode="r:*") as tar:
        generated_paths = json.loads(tar.extractfile(".generated_paths.json").read())
    return generated_paths


def docker_container_create(prefix, task) -> str:
    codegen_dir = Path("codegen")
    spec_dir = Path("spec")
    container_name = "codegen-{}-{}-{}".format(
        task["language"],
        task["language_task_index"] + 1,
        "".join(random.choice(string.ascii_lowercase) for _ in range(10)),
    )
    template_path = codegen_dir.joinpath(task["template"])
    template_dir = template_path.parent

    # Build input file mounts - files can be in codegen/ or spec/
    input_file_mounts = []
    for f in task["input_files"]:
        # Check if file exists in codegen/ or spec/
        if codegen_dir.joinpath(f).exists():
            src = codegen_dir.joinpath(f).absolute()
        elif spec_dir.joinpath(f).exists():
            src = spec_dir.joinpath(f).absolute()
        else:
            print(f"{YELLOW}Warning: input file {f} not found in codegen/ or spec/{ENDC}")
            src = spec_dir.joinpath(f).absolute()  # Default to spec/
        input_file_mounts.append(f"--mount=type=bind,src={src},dst=/app/{f},ro")

    cmd = [
        get_docker_binary(),
        "container",
        "run",
        "-d",
        "--name",
        container_name,
        "--workdir",
        "/app",
        *input_file_mounts,
        "--mount",
        f"type=bind,src={template_dir.absolute()},dst=/app/{template_dir},ro",
    ]

    for extra_mount_src, extra_mount_dst in task["extra_mounts"].items():
        src_path = Path(extra_mount_src)
        if src_path.exists():
            cmd.append("--mount")
            cmd.append(
                f"type=bind,src={src_path.absolute()},dst={extra_mount_dst},ro"
            )
        else:
            dbg(prefix, f"{YELLOW}Skipping mount {extra_mount_src} (not found){ENDC}")

    input_file_args = [f"--input-file={f}" for f in task["input_files"]]
    cmd.extend(
        [
            OPENAPI_CODEGEN_IMAGE,
            "openapi-codegen",
            "generate",
            *task["extra_codegen_args"],
            f"--template={template_path}",
            *input_file_args,
            f"--output-dir={task['output_dir']}",
        ]
    )
    run_cmd(prefix, cmd)
    return container_name


def run_cmd(prefix, cmd, dont_dbg=False) -> subprocess.CompletedProcess[bytes]:
    dbg_cmd = [cmd[0], *[f'"{i}"' for i in cmd[1:]]]
    dbg(prefix, f"{BLUE}Running command{ENDC} {' '.join(dbg_cmd)}")
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode != 0:
        print_cmd_result(result, prefix)
        prefix_print(
            prefix,
            f"subprocess exited with code {result.returncode} (run with DEBUG=yes to see extra logs)",
        )
        raise ExitException(result.returncode)

    if DEBUG and not dont_dbg:
        print_cmd_result(result, prefix)

    return result


def print_cmd_result(result: subprocess.CompletedProcess[bytes], prefix: str):
    for i, stream in enumerate([result.stdout, result.stderr]):
        output = stream.decode("utf-8")
        if output != "":
            cli_prefix = f"{CYAN}{'stdout' if i == 0 else 'stderr'}{ENDC} "
            nice_logs = "{}{}".format(
                cli_prefix,
                output.strip().replace("\n", f"\n{cli_prefix}"),
            )
            prefix_print(prefix, nice_logs)


def dbg(prefix, msg):
    if not DEBUG:
        return
    prefix_print(prefix, msg)


def prefix_print(prefix, msg):
    print(
        "{}{}".format(f"{prefix.strip()} ", msg.replace("\n", f"\n{prefix.strip()} ")),
        flush=True,
    )


def execute_codegen_task(task) -> list[str]:
    prefix = "{}{} ({}/{}){} | ".format(
        GREEN,
        task["language"],
        task["language_task_index"] + 1,
        task["language_total"],
        ENDC,
    )

    container_name = docker_container_create(prefix, task).strip()
    dbg(prefix, f"Container id {container_name}")

    exit_code = docker_container_wait(prefix, container_name)

    logs = docker_container_logs(prefix, container_name)

    nice_logs = "{}{}".format(
        f"{CYAN}container logs{ENDC} ",
        logs.strip().replace("\n", f"\n{CYAN}container logs{ENDC} "),
    )

    if exit_code != 0:
        prefix_print(prefix, nice_logs)
        prefix_print(
            prefix,
            f"subprocess exited with code {exit_code} (run with DEBUG=yes to see extra logs)",
        )
        raise ExitException(exit_code)

    dbg(prefix, nice_logs)

    docker_container_cp(prefix, container_name, task)

    generated_paths = get_generated_paths(prefix, container_name, task)

    docker_container_rm(prefix, container_name)

    print(f"{GREEN}#{ENDC}", flush=True, end="")

    return generated_paths


def run_codegen_for_language(language, language_config) -> list[str]:
    tasks_results = []
    with ThreadPoolExecutor(max_workers=multiprocessing.cpu_count()) as pool:
        futures = []
        for t in language_config["tasks"]:
            futures.append(pool.submit(execute_codegen_task, t))
        for future in as_completed(futures):
            if future.exception() is not None:
                raise future.exception()
            else:
                tasks_results.append(future.result())

    extra_shell_commands = language_config.get("extra_shell_commands", [])
    for index, shell_command in enumerate(extra_shell_commands):
        cmd = ["bash", "-c", shell_command]
        run_cmd(
            f"{GREEN}{language}[extra shell commands] ({index + 1}/{len(extra_shell_commands)}){ENDC} | ",
            cmd,
        )
    return tasks_results


def parse_config():
    with open(Path("codegen").joinpath("codegen.toml"), "rb") as f:
        data = tomllib.load(f)
    input_files = data.pop("global")["input_files"]
    config = {}
    for language, language_config in data.items():
        config[language] = {"tasks": [], "tasks_count": len(language_config["task"])}
        for language_task_index, task in enumerate(language_config["task"]):
            config[language]["extra_shell_commands"] = language_config.get(
                "extra_shell_commands", []
            )
            config[language]["tasks"].append(
                {
                    "language": language,
                    "language_task_index": language_task_index,
                    "language_total": len(language_config.get("task", [])),
                    "input_files": task.get(
                        "input_files", language_config.get("input_files", input_files)
                    ),
                    "template": task["template"],
                    "output_dir": task["output_dir"],
                    "extra_mounts": language_config.get("extra_mounts", {}),
                    "extra_codegen_args": task.get(
                        "extra_codegen_args",
                        language_config.get("extra_codegen_args", []),
                    ),
                }
            )

    if DEBUG:
        print(json.dumps(config, indent=4), flush=True)
    return config


def check_image_exists() -> bool:
    """Check if the codegen Docker image exists locally."""
    result = subprocess.run(
        [
            get_docker_binary(),
            "image",
            "inspect",
            "--format",
            "ignore me",
            OPENAPI_CODEGEN_IMAGE,
        ],
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
    )
    return result.returncode == 0


def log_generated_files(generated_paths: list[list[str]]):
    processed_files = set()

    for files_generated_in_a_single_task in generated_paths:
        for path in files_generated_in_a_single_task:
            if path in processed_files:
                raise Exception(
                    f"Unexpected: same file `{path}` was generated twice in 2 different codegen tasks"
                )
            processed_files.add(path)

            # Ensure each file has `this file is @generated` in the first (3) lines
            with open(path, "r") as f:
                first_lines = "".join(f.readlines()[0:3])
                assert "@generated" in first_lines.lower(), (
                    f"Missing the `this file is @generated` comment in {path}"
                )

    # Check for orphaned generated files
    files_not_part_of_codegen = []
    cmd = ["git", "grep", "-i", "-l", "this file is @generated", "--", ":!codegen/*"]
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode == 0:
        matched_files = result.stdout.decode("utf-8").splitlines()
        for m in matched_files:
            if m not in processed_files and m != "regen_openapi.py":
                files_not_part_of_codegen.append(m)

        if len(files_not_part_of_codegen) != 0:
            print(f"{YELLOW}Warning: Found {len(files_not_part_of_codegen)} files on disk that were not generated by the codegen:{ENDC}")
            for f in files_not_part_of_codegen:
                print(f"  - {f}")

    # Sort and save generated files list
    for i in generated_paths:
        i.sort()
    generated_paths.sort()
    with open(Path("codegen").joinpath("generated_files.json"), "w") as f:
        json.dump(generated_paths, f, indent=4)
        f.write("\n")


def main():
    repo_root = Path(__file__).parent.resolve()
    os.chdir(repo_root)

    if not check_image_exists():
        print(f"{YELLOW}Docker image '{OPENAPI_CODEGEN_IMAGE}' not found.{ENDC}")
        print(f"Please build it first with:")
        print(f"  cd codegen && docker build -t {OPENAPI_CODEGEN_IMAGE} .")
        exit(1)

    config = parse_config()
    all_tasks = sum([i["tasks_count"] for i in config.values()])
    print(f"Running {all_tasks} codegen tasks")

    start = time.perf_counter()
    exit_with_error = False
    tasks_results = []
    with ThreadPoolExecutor(max_workers=multiprocessing.cpu_count()) as pool:
        futures = []
        for language, language_config in config.items():
            futures.append(
                pool.submit(run_codegen_for_language, language, language_config)
            )
        for future in as_completed(futures):
            if future.exception() is not None:
                if isinstance(future.exception(), ExitException):
                    exit_with_error = True
                else:
                    raise future.exception()
            else:
                tasks_results.extend(future.result())

    print()  # Final newline
    if exit_with_error:
        exit(1)
    else:
        log_generated_files(tasks_results)
        end = time.perf_counter()
        length = end - start
        print(f"Ran {all_tasks} codegen tasks in {round(length, 4)} seconds")


if __name__ == "__main__":
    main()
