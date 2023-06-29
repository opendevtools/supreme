{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    [
      "@semantic-release/exec",
      {
        "verifyConditionsCmd": "semantic-release-rust verify-conditions",
        "prepareCmd": "semantic-release-rust prepare ${nextRelease.version}"
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": ["Cargo.toml", "Cargo.lock", "CHANGELOG.md"]
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": [
          {
            "path": "artifacts/aarch64-apple-darwin/{{name}}-aarch64-apple-darwin.tar.gz",
            "label": "{{name}}-${nextRelease.version}-aarch64-apple-darwin.tar.gz",
            "name": "{{name}}-${nextRelease.version}-aarch64-apple-darwin.tar.gz"
          }
        ]
      }
    ]
  ]
}

