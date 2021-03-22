{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    {{#unless noNpm}}"@semantic-release/npm", {{else}}["@semantic-release/npm", {
      "npmPublish": false
    }],{{/unless}}
    "@semantic-release/git"
  ]
}

