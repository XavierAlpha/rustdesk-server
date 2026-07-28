#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: debian/build-packages.sh --architecture ARCH --workspace DIRECTORY

Wrap the prebuilt static hbbs, hbbr, and rustdesk-utils binaries in the three
Debian packages declared by this repository. DIRECTORY must already contain a
bin/ directory with those three regular files and must not contain debian/.
EOF
}

fail() {
  echo "debian package build: $*" >&2
  exit 1
}

architecture=""
workspace=""
while (( $# > 0 )); do
  case "$1" in
    --architecture)
      (( $# >= 2 )) || fail "--architecture requires a value"
      architecture="$2"
      shift 2
      ;;
    --workspace)
      (( $# >= 2 )) || fail "--workspace requires a value"
      workspace="$2"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      fail "unknown argument: $1"
      ;;
  esac
done

[[ -n "$architecture" ]] || fail "--architecture is required"
[[ -n "$workspace" ]] || fail "--workspace is required"
case "$architecture" in
  amd64) expected_elf_machine="Advanced Micro Devices X86-64" ;;
  arm64) expected_elf_machine="AArch64" ;;
  armhf) expected_elf_machine="ARM" ;;
  i386) expected_elf_machine="Intel 80386" ;;
  *)
    fail "unsupported Debian architecture: $architecture"
    ;;
esac

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(realpath --canonicalize-existing -- "$script_dir/..")"
workspace="$(realpath --canonicalize-existing -- "$workspace")"
package_directory="$(dirname -- "$workspace")"

[[ "$workspace" != "$repository_root" ]] || fail "workspace must not be the repository root"
case "$workspace/" in
  "$script_dir/"*)
    fail "workspace must not be inside the Debian source directory"
    ;;
esac
[[ ! -e "$workspace/debian" ]] || fail "workspace already contains debian/"

binary_names=(hbbs hbbr rustdesk-utils)
for binary_name in "${binary_names[@]}"; do
  binary_path="$workspace/bin/$binary_name"
  [[ -f "$binary_path" && ! -L "$binary_path" ]] \
    || fail "missing regular, non-symlink binary: $binary_path"
  chmod 0755 "$binary_path"

  elf_header="$(LC_ALL=C readelf --file-header --wide "$binary_path" 2>/dev/null)" \
    || fail "binary is not a valid ELF file: $binary_path"
  elf_machine="$(
    sed -n 's/^[[:space:]]*Machine:[[:space:]]*//p' <<< "$elf_header"
  )"
  [[ "$elf_machine" == "$expected_elf_machine" ]] \
    || fail "binary architecture mismatch for $binary_path: expected $expected_elf_machine, got $elf_machine"

  dynamic_section="$(LC_ALL=C readelf --dynamic --wide "$binary_path" 2>/dev/null)" \
    || fail "cannot inspect dynamic dependencies: $binary_path"
  if grep -Fq '(NEEDED)' <<< "$dynamic_section"; then
    fail "binary has dynamic library dependencies: $binary_path"
  fi
done

cp -a -- "$script_dir" "$workspace/debian"
grep -Fq '{{ ARCH }}' "$script_dir/control.tpl" \
  || fail "debian/control.tpl has no architecture placeholder"
sed "s/{{ ARCH }}/${architecture}/g" "$script_dir/control.tpl" \
  > "$workspace/debian/control"
if grep -Fq '{{ ARCH }}' "$workspace/debian/control"; then
  fail "generated Debian control file still contains an architecture placeholder"
fi

(
  cd -- "$workspace"
  dpkg-buildpackage --build=binary --no-sign -a"$architecture"
)

mapfile -d '' packages < <(
  find "$package_directory" -maxdepth 1 -type f \
    -name "rustdesk-server-*_${architecture}.deb" -print0 \
    | sort -z
)
[[ "${#packages[@]}" -eq 3 ]] \
  || fail "expected three ${architecture} packages, found ${#packages[@]}"

declare -A expected_packages=(
  [rustdesk-server-hbbs]=1
  [rustdesk-server-hbbr]=1
  [rustdesk-server-utils]=1
)
hbbs_package=""
hbbr_package=""
utils_package=""
for package in "${packages[@]}"; do
  dpkg-deb --info "$package" >/dev/null
  package_name="$(dpkg-deb --field "$package" Package)"
  [[ "${expected_packages[$package_name]:-}" == "1" ]] \
    || fail "unexpected or duplicate Debian package: $package_name"
  unset 'expected_packages[$package_name]'
  case "$package_name" in
    rustdesk-server-hbbs) hbbs_package="$package" ;;
    rustdesk-server-hbbr) hbbr_package="$package" ;;
    rustdesk-server-utils) utils_package="$package" ;;
  esac
done
[[ "${#expected_packages[@]}" -eq 0 ]] || fail "one or more Debian packages are missing"

dpkg-deb --contents "$hbbs_package" \
  | grep -E '\./(usr/)?lib/systemd/system/rustdesk-hbbs\.service$' >/dev/null
dpkg-deb --contents "$hbbr_package" \
  | grep -E '\./(usr/)?lib/systemd/system/rustdesk-hbbr\.service$' >/dev/null
dpkg-deb --fsys-tarfile "$hbbs_package" \
  | tar -xOf - ./usr/bin/hbbs \
  | cmp - "$workspace/bin/hbbs"
dpkg-deb --fsys-tarfile "$hbbr_package" \
  | tar -xOf - ./usr/bin/hbbr \
  | cmp - "$workspace/bin/hbbr"
dpkg-deb --fsys-tarfile "$utils_package" \
  | tar -xOf - ./usr/bin/rustdesk-utils \
  | cmp - "$workspace/bin/rustdesk-utils"

printf 'Built and verified Debian packages for %s:\n' "$architecture"
printf '  %s\n' "${packages[@]}"
