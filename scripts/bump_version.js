const { readFileSync, writeFileSync } = require('fs');
const { join } = require('path');
const semver = require('semver')

const versionFilePath = ".version";

const filesPaths = [
    versionFilePath, 
    // Rust Client
    "rust/Cargo.toml",
    "rust/README.md",
    // Java
    "java/gradle.properties",
    "java/README.md",
    "java/src/main/java/com/afinsp/Version.java"
];

const rootDir = join(__dirname, "..");

if (process.argv.length !== 3 || !semver.valid(process.argv[2])) {
    console.error("must supply a valid semantic version number");
    return;
}
const newVersion = process.argv[2];
const currentVersion = readFileSync(join(rootDir, versionFilePath), 'utf8').trim();

if (semver.lte(newVersion, currentVersion)) {
    console.error("supplied version must be greater than current version");
    return;
}

const replaceRegExp = new RegExp(currentVersion, 'g');

// Update Version Files
filesPaths.forEach((relativePath) => {
    const filePath = join(rootDir, relativePath);
    const content = readFileSync(filePath, 'utf8');
    const updated_content = content.replace(replaceRegExp, newVersion);
    if (content === updated_content) {
        console.error("New version was not written to " + filePath)
        process.exit(1);
    }
    writeFileSync(filePath, updated_content);
})

// Add Changelog Section
const changelogPath = join(rootDir, "CHANGELOG.md");
const changelogUpdate = readFileSync(changelogPath, 'utf8').replace("# Changelog\n\n## Next", `# Changelog\n\n## Next\n* \n\n## Version ${newVersion}`);
writeFileSync(changelogPath, changelogUpdate);

console.log("Version bumped from %s to %s, don't forget to update the changelog!", currentVersion, newVersion);
