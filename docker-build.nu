# Due to difficulties building cross-platform binaries with Docker,
# I resort here to building locally using `cross` and then copying the
# output into a container.

# Ensure 'cross' is installed
if (which cross | is-empty) {
    error make 'Please install "cross" to continue.'
}

# Ensure a container engine is installed
if (which docker | is-empty) {
    error make 'Please install "docker" to continue.'
}

# Get name and version from the manifest
let manifest = open Cargo.toml

let name = $manifest.package.name
let version = $manifest.package.version

# List build targets
let targets = [
    { target: 'arm-unknown-linux-gnueabihf', docker_platform: 'linux/arm/v6' },
    { target: 'armv7-unknown-linux-gnueabihf', docker_platform: 'linux/arm/v7' },
    { target: 'aarch64-unknown-linux-musl', docker_platform: 'linux/arm64' },
    { target: 'x86_64-unknown-linux-musl', docker_platform: 'linux/amd64' }
]

# Build for every image
let build_dir = 'target/building-for-docker'

for entry in $targets {
    let target = $entry.target
    let docker_platform = $entry.docker_platform

    print $"\nBuilding for target: ($target)...\n"

    # We use a different target directory for each crate and each target, otherwise dependencies build
    # may clash between platforms
    let target_dir = $"($build_dir)/targets/($target)"
    mkdir $target_dir
    cross build --release --target $target --target-dir $target_dir

    # Put build artifact in the correct directory
    let artifacts_file = $"($build_dir)/artifacts/($docker_platform)/($name)"
    mkdir ($artifacts_file | path dirname)
    cp $"($target_dir)/($target)/release/($name)" $artifacts_file
}

# Build and publish Docker images
print "\nPublishing on Docker Hub...\n"

# Build and publish the image for all platforms
docker buildx build --push . --platform ($targets | each { $in.docker_platform } | str join ',') --tag $"clementnerma/($name):($version)" --tag $"clementnerma/($name):latest"
