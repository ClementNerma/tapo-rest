# Due to difficulties building cross-platform binaries with Docker,
# I resort here to building locally using `cross` and then copying the
# output into a container.

# Ensure 'cross' is installed
if whereis('cross') == null {
    throw 'Please install "cross" to continue.'
}

# Ensure a container engine is installed
if whereis('docker') == null && whereis('podman') == null {
    throw 'Please install either "docker" or "podman" to continue.'
}

# Get name and version from the manifest
let manifest = readFile('Cargo.toml').parseToml()
let { name, version } = $manifest.package

# List build targets
let targets = map {
    'arm-unknown-linux-gnueabihf': 'linux/arm/v6',
    'armv7-unknown-linux-gnueabihf': 'linux/arm/v7',
    'aarch64-unknown-linux-musl': 'linux/arm64',
    'x86_64-unknown-linux-musl': 'linux/amd64'
}

# Build for every image
let buildDir = 'target/building-for-docker'

for target, dockerPlatform in $targets {
    echo "\nBuilding for target: $target...\n"

    # We use a different target directory for each crate and each target, otherwise dependencies build
    # may clash between platforms
    let targetDir = "$buildDir/targets/$target"
    mkdir -p -i $targetDir
    cross build --release --target $target --target-dir $targetDir

    # Put build artifact in the correct directory
    let artifactsFile = "$buildDir/artifacts/$dockerPlatform/$name"
    mkdir -p (parentDir($artifactsFile))
    cp "$targetDir/$target/release/$name" $artifactsFile
}

# Build and publish Docker images
echo '\nPublishing on Docker Hub...\n'

docker buildx build --push . \
    --platform ($targets.values().join(',')) \
    --tag "clementnerma/$name:$version" \
    --tag "clementnerma/$name:latest"
