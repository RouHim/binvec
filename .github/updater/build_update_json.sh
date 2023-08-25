#!/usr/bin/env bash
# shellcheck disable=SC2046
#
# Description:
#   This script is used to build the update json file.
#
# Parameters:
#   $1: The version of the release without v
#
# Example:
#   ./build_update_json.sh 0.1.0
#
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
set -e

VERSION=$1
UPDATER_JSON_FILE=$2
NOW=$(date -u +%Y-%m-%dT%H:%M:%SZ)
NOTES=$(jq -r '.head_commit.message' "$GITHUB_EVENT_PATH")
REPO_BASE_URL="https://github.com/RouHim/binvec"

echo "Version: $VERSION"
echo "Updater JSON file: $UPDATER_JSON_FILE"
echo "NOTES: $NOTES"
echo "Repo: $REPO_BASE_URL"

# Collect linux AppImage data
APPIMAGE_URL="${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}_amd64.AppImage.tar.gz"
APPIMAGE_SIG=$(curl -L --silent "${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}_amd64.AppImage.tar.gz.sig")

# Collect windows MSI data
MSI_URL=${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}_x64_en-US.msi.zip
MSI_SIG=$(curl -L --silent "${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}_x64_en-US.msi.zip.sig")

# Collect macos APP data
APP_URL=${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}.app.tar.gz
APP_SIG=$(curl -L --silent "${REPO_BASE_URL}/releases/download/v${VERSION}/binvec_${VERSION}.app.tar.gz.sig")

# Set the generic version, notes and pub_date
cat <<<$(jq --arg VERSION "$VERSION" '.version = "\($VERSION)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}
cat <<<$(jq --arg NOTES "$NOTES" '.notes = "\($NOTES)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}
cat <<<$(jq --arg NOW "$NOW" '.pub_date = "\($NOW)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}

# Set the linux specific data
cat <<<$(jq --arg APPIMAGE_URL "$APPIMAGE_URL" '.platforms."linux-x86_64".url = "\($APPIMAGE_URL)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}
cat <<<$(jq --arg APPIMAGE_SIG "$APPIMAGE_SIG" '.platforms."linux-x86_64".signature = "\($APPIMAGE_SIG)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}

# Set the windows specific data
cat <<<$(jq --arg MSI_URL "$MSI_URL" '.platforms."windows-x86_64".url = "\($MSI_URL)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}
cat <<<$(jq --arg MSI_SIG "$MSI_SIG" '.platforms."windows-x86_64".signature = "\($MSI_SIG)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}

# Set the macos specific data
cat <<<$(jq --arg APP_URL "$APP_URL" '.platforms."darwin-x86_64".url = "\($APP_URL)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}
cat <<<$(jq --arg APP_SIG "$APP_SIG" '.platforms."darwin-x86_64".signature = "\($APP_SIG)"' ${UPDATER_JSON_FILE}) >${UPDATER_JSON_FILE}

# Print the final json file
cat "${UPDATER_JSON_FILE}"
