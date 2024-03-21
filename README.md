# What

In order to download the build artifacts, you will need to create a fine-grained personal access token on Github. Read how [here](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token).

Once you have a PAT, you can list the artifacts with the following command:

```bash
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR_GITHUB_PAT>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/OWNER/REPO/actions/artifacts
```

You can download the artifact using the `archive_download_url` from the previous command in the following command:

```bash
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR_GITHUB_PAT>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  <archive_download_url> --output target/debug/build_d73e35faf4a73e206c31c0a3ee799aa398ba370e.zip
```

After downloading, be sure to grant execution permissions to the executable before running it:

```bash
chmod +x target/debug/build_d73e35faf4a73e206c31c0a3ee799aa398ba370e.zip
```