---
editUrl: false
next: false
prev: false
title: "GitHubClient"
---

Defined in: [packages/agentic-control/src/github/client.ts:58](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L58)

## Constructors

### Constructor

> **new GitHubClient**(`config?`): `GitHubClient`

Defined in: [packages/agentic-control/src/github/client.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L70)

Create a new GitHubClient.

Can be used in two modes:
1. With explicit config: new GitHubClient({ token, owner, repo })
2. Token-aware mode: new GitHubClient() - uses token based on repo

#### Parameters

##### config?

`GitHubClientConfig` = `{}`

#### Returns

`GitHubClient`

## Methods

### collectFeedback()

> **collectFeedback**(`prNumber`): `Promise`\<`object`[]\>

Defined in: [packages/agentic-control/src/github/client.ts:598](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L598)

Collect all feedback on a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<`object`[]\>

***

### getCIStatus()

> **getCIStatus**(`prNumber`): `Promise`\<\{ `allPassing`: `boolean`; `anyPending`: `boolean`; `checks`: `object`[]; `failures`: `object`[]; \}\>

Defined in: [packages/agentic-control/src/github/client.ts:549](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L549)

Get CI status for a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<\{ `allPassing`: `boolean`; `anyPending`: `boolean`; `checks`: `object`[]; `failures`: `object`[]; \}\>

***

### getIssueComments()

> **getIssueComments**(`prNumber`): `Promise`\<`object`[]\>

Defined in: [packages/agentic-control/src/github/client.ts:399](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L399)

Get issue comments on a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<`object`[]\>

***

### getPR()

> **getPR**(`prNumber`): `Promise`\<\{ `_links`: \{ `comments`: \{ `href`: `string`; \}; `commits`: \{ `href`: `string`; \}; `html`: \{ `href`: `string`; \}; `issue`: \{ `href`: `string`; \}; `review_comment`: \{ `href`: `string`; \}; `review_comments`: \{ `href`: `string`; \}; `self`: \{ `href`: `string`; \}; `statuses`: \{ `href`: `string`; \}; \}; `active_lock_reason?`: `string` \| `null`; `additions`: `number`; `assignee`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `assignees?`: `object`[] \| `null`; `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `auto_merge`: \{ `commit_message`: `string`; `commit_title`: `string`; `enabled_by`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `merge_method`: `"merge"` \| `"squash"` \| `"rebase"`; \} \| `null`; `base`: \{ `label`: `string`; `ref`: `string`; `repo`: \{ `allow_auto_merge?`: `boolean`; `allow_forking?`: `boolean`; `allow_merge_commit?`: `boolean`; `allow_rebase_merge?`: `boolean`; `allow_squash_merge?`: `boolean`; `allow_update_branch?`: `boolean`; `anonymous_access_enabled?`: `boolean`; `archive_url`: `string`; `archived`: `boolean`; `assignees_url`: `string`; `blobs_url`: `string`; `branches_url`: `string`; `clone_url`: `string`; `code_search_index_status?`: \{ `lexical_commit_sha?`: `string`; `lexical_search_ok?`: `boolean`; \}; `collaborators_url`: `string`; `comments_url`: `string`; `commits_url`: `string`; `compare_url`: `string`; `contents_url`: `string`; `contributors_url`: `string`; `created_at`: `string` \| `null`; `default_branch`: `string`; `delete_branch_on_merge?`: `boolean`; `deployments_url`: `string`; `description`: `string` \| `null`; `disabled`: `boolean`; `downloads_url`: `string`; `events_url`: `string`; `fork`: `boolean`; `forks`: `number`; `forks_count`: `number`; `forks_url`: `string`; `full_name`: `string`; `git_commits_url`: `string`; `git_refs_url`: `string`; `git_tags_url`: `string`; `git_url`: `string`; `has_discussions?`: `boolean`; `has_downloads`: `boolean`; `has_issues`: `boolean`; `has_pages`: `boolean`; `has_projects`: `boolean`; `has_wiki`: `boolean`; `homepage`: `string` \| `null`; `hooks_url`: `string`; `html_url`: `string`; `id`: `number`; `is_template?`: `boolean`; `issue_comment_url`: `string`; `issue_events_url`: `string`; `issues_url`: `string`; `keys_url`: `string`; `labels_url`: `string`; `language`: `string` \| `null`; `languages_url`: `string`; `license`: \{ `html_url?`: `string`; `key`: `string`; `name`: `string`; `node_id`: `string`; `spdx_id`: `string` \| `null`; `url`: `string` \| `null`; \} \| `null`; `master_branch?`: `string`; `merge_commit_message?`: `"PR_TITLE"` \| `"PR_BODY"` \| `"BLANK"`; `merge_commit_title?`: `"PR_TITLE"` \| `"MERGE_MESSAGE"`; `merges_url`: `string`; `milestones_url`: `string`; `mirror_url`: `string` \| `null`; `name`: `string`; `node_id`: `string`; `notifications_url`: `string`; `open_issues`: `number`; `open_issues_count`: `number`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `permissions?`: \{ `admin`: `boolean`; `maintain?`: `boolean`; `pull`: `boolean`; `push`: `boolean`; `triage?`: `boolean`; \}; `private`: `boolean`; `pulls_url`: `string`; `pushed_at`: `string` \| `null`; `releases_url`: `string`; `size`: `number`; `squash_merge_commit_message?`: `"PR_BODY"` \| `"COMMIT_MESSAGES"` \| `"BLANK"`; `squash_merge_commit_title?`: `"PR_TITLE"` \| `"COMMIT_OR_PR_TITLE"`; `ssh_url`: `string`; `stargazers_count`: `number`; `stargazers_url`: `string`; `starred_at?`: `string`; `statuses_url`: `string`; `subscribers_url`: `string`; `subscription_url`: `string`; `svn_url`: `string`; `tags_url`: `string`; `teams_url`: `string`; `temp_clone_token?`: `string`; `topics?`: `string`[]; `trees_url`: `string`; `updated_at`: `string` \| `null`; `url`: `string`; `use_squash_pr_title_as_default?`: `boolean`; `visibility?`: `string`; `watchers`: `number`; `watchers_count`: `number`; `web_commit_signoff_required?`: `boolean`; \}; `sha`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}; `body`: `string` \| `null`; `changed_files`: `number`; `closed_at`: `string` \| `null`; `comments`: `number`; `comments_url`: `string`; `commits`: `number`; `commits_url`: `string`; `created_at`: `string`; `deletions`: `number`; `diff_url`: `string`; `draft?`: `boolean`; `head`: \{ `label`: `string`; `ref`: `string`; `repo`: \{ `allow_auto_merge?`: `boolean`; `allow_forking?`: `boolean`; `allow_merge_commit?`: `boolean`; `allow_rebase_merge?`: `boolean`; `allow_squash_merge?`: `boolean`; `allow_update_branch?`: `boolean`; `anonymous_access_enabled?`: `boolean`; `archive_url`: `string`; `archived`: `boolean`; `assignees_url`: `string`; `blobs_url`: `string`; `branches_url`: `string`; `clone_url`: `string`; `code_search_index_status?`: \{ `lexical_commit_sha?`: `string`; `lexical_search_ok?`: `boolean`; \}; `collaborators_url`: `string`; `comments_url`: `string`; `commits_url`: `string`; `compare_url`: `string`; `contents_url`: `string`; `contributors_url`: `string`; `created_at`: `string` \| `null`; `default_branch`: `string`; `delete_branch_on_merge?`: `boolean`; `deployments_url`: `string`; `description`: `string` \| `null`; `disabled`: `boolean`; `downloads_url`: `string`; `events_url`: `string`; `fork`: `boolean`; `forks`: `number`; `forks_count`: `number`; `forks_url`: `string`; `full_name`: `string`; `git_commits_url`: `string`; `git_refs_url`: `string`; `git_tags_url`: `string`; `git_url`: `string`; `has_discussions?`: `boolean`; `has_downloads`: `boolean`; `has_issues`: `boolean`; `has_pages`: `boolean`; `has_projects`: `boolean`; `has_wiki`: `boolean`; `homepage`: `string` \| `null`; `hooks_url`: `string`; `html_url`: `string`; `id`: `number`; `is_template?`: `boolean`; `issue_comment_url`: `string`; `issue_events_url`: `string`; `issues_url`: `string`; `keys_url`: `string`; `labels_url`: `string`; `language`: `string` \| `null`; `languages_url`: `string`; `license`: \{ `html_url?`: `string`; `key`: `string`; `name`: `string`; `node_id`: `string`; `spdx_id`: `string` \| `null`; `url`: `string` \| `null`; \} \| `null`; `master_branch?`: `string`; `merge_commit_message?`: `"PR_TITLE"` \| `"PR_BODY"` \| `"BLANK"`; `merge_commit_title?`: `"PR_TITLE"` \| `"MERGE_MESSAGE"`; `merges_url`: `string`; `milestones_url`: `string`; `mirror_url`: `string` \| `null`; `name`: `string`; `node_id`: `string`; `notifications_url`: `string`; `open_issues`: `number`; `open_issues_count`: `number`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `permissions?`: \{ `admin`: `boolean`; `maintain?`: `boolean`; `pull`: `boolean`; `push`: `boolean`; `triage?`: `boolean`; \}; `private`: `boolean`; `pulls_url`: `string`; `pushed_at`: `string` \| `null`; `releases_url`: `string`; `size`: `number`; `squash_merge_commit_message?`: `"PR_BODY"` \| `"COMMIT_MESSAGES"` \| `"BLANK"`; `squash_merge_commit_title?`: `"PR_TITLE"` \| `"COMMIT_OR_PR_TITLE"`; `ssh_url`: `string`; `stargazers_count`: `number`; `stargazers_url`: `string`; `starred_at?`: `string`; `statuses_url`: `string`; `subscribers_url`: `string`; `subscription_url`: `string`; `svn_url`: `string`; `tags_url`: `string`; `teams_url`: `string`; `temp_clone_token?`: `string`; `topics?`: `string`[]; `trees_url`: `string`; `updated_at`: `string` \| `null`; `url`: `string`; `use_squash_pr_title_as_default?`: `boolean`; `visibility?`: `string`; `watchers`: `number`; `watchers_count`: `number`; `web_commit_signoff_required?`: `boolean`; \}; `sha`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}; `html_url`: `string`; `id`: `number`; `issue_url`: `string`; `labels`: `object`[]; `locked`: `boolean`; `maintainer_can_modify`: `boolean`; `merge_commit_sha`: `string` \| `null`; `mergeable`: `boolean` \| `null`; `mergeable_state`: `string`; `merged`: `boolean`; `merged_at`: `string` \| `null`; `merged_by`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `milestone`: \{ `closed_at`: `string` \| `null`; `closed_issues`: `number`; `created_at`: `string`; `creator`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `description`: `string` \| `null`; `due_on`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `labels_url`: `string`; `node_id`: `string`; `number`: `number`; `open_issues`: `number`; `state`: `"open"` \| `"closed"`; `title`: `string`; `updated_at`: `string`; `url`: `string`; \} \| `null`; `node_id`: `string`; `number`: `number`; `patch_url`: `string`; `rebaseable?`: `boolean` \| `null`; `requested_reviewers?`: `object`[] \| `null`; `requested_teams?`: `object`[] \| `null`; `review_comment_url`: `string`; `review_comments`: `number`; `review_comments_url`: `string`; `state`: `"open"` \| `"closed"`; `statuses_url`: `string`; `title`: `string`; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}\>

Defined in: [packages/agentic-control/src/github/client.ts:241](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L241)

Get pull request (instance version with full data)

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<\{ `_links`: \{ `comments`: \{ `href`: `string`; \}; `commits`: \{ `href`: `string`; \}; `html`: \{ `href`: `string`; \}; `issue`: \{ `href`: `string`; \}; `review_comment`: \{ `href`: `string`; \}; `review_comments`: \{ `href`: `string`; \}; `self`: \{ `href`: `string`; \}; `statuses`: \{ `href`: `string`; \}; \}; `active_lock_reason?`: `string` \| `null`; `additions`: `number`; `assignee`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `assignees?`: `object`[] \| `null`; `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `auto_merge`: \{ `commit_message`: `string`; `commit_title`: `string`; `enabled_by`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `merge_method`: `"merge"` \| `"squash"` \| `"rebase"`; \} \| `null`; `base`: \{ `label`: `string`; `ref`: `string`; `repo`: \{ `allow_auto_merge?`: `boolean`; `allow_forking?`: `boolean`; `allow_merge_commit?`: `boolean`; `allow_rebase_merge?`: `boolean`; `allow_squash_merge?`: `boolean`; `allow_update_branch?`: `boolean`; `anonymous_access_enabled?`: `boolean`; `archive_url`: `string`; `archived`: `boolean`; `assignees_url`: `string`; `blobs_url`: `string`; `branches_url`: `string`; `clone_url`: `string`; `code_search_index_status?`: \{ `lexical_commit_sha?`: `string`; `lexical_search_ok?`: `boolean`; \}; `collaborators_url`: `string`; `comments_url`: `string`; `commits_url`: `string`; `compare_url`: `string`; `contents_url`: `string`; `contributors_url`: `string`; `created_at`: `string` \| `null`; `default_branch`: `string`; `delete_branch_on_merge?`: `boolean`; `deployments_url`: `string`; `description`: `string` \| `null`; `disabled`: `boolean`; `downloads_url`: `string`; `events_url`: `string`; `fork`: `boolean`; `forks`: `number`; `forks_count`: `number`; `forks_url`: `string`; `full_name`: `string`; `git_commits_url`: `string`; `git_refs_url`: `string`; `git_tags_url`: `string`; `git_url`: `string`; `has_discussions?`: `boolean`; `has_downloads`: `boolean`; `has_issues`: `boolean`; `has_pages`: `boolean`; `has_projects`: `boolean`; `has_wiki`: `boolean`; `homepage`: `string` \| `null`; `hooks_url`: `string`; `html_url`: `string`; `id`: `number`; `is_template?`: `boolean`; `issue_comment_url`: `string`; `issue_events_url`: `string`; `issues_url`: `string`; `keys_url`: `string`; `labels_url`: `string`; `language`: `string` \| `null`; `languages_url`: `string`; `license`: \{ `html_url?`: `string`; `key`: `string`; `name`: `string`; `node_id`: `string`; `spdx_id`: `string` \| `null`; `url`: `string` \| `null`; \} \| `null`; `master_branch?`: `string`; `merge_commit_message?`: `"PR_TITLE"` \| `"PR_BODY"` \| `"BLANK"`; `merge_commit_title?`: `"PR_TITLE"` \| `"MERGE_MESSAGE"`; `merges_url`: `string`; `milestones_url`: `string`; `mirror_url`: `string` \| `null`; `name`: `string`; `node_id`: `string`; `notifications_url`: `string`; `open_issues`: `number`; `open_issues_count`: `number`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `permissions?`: \{ `admin`: `boolean`; `maintain?`: `boolean`; `pull`: `boolean`; `push`: `boolean`; `triage?`: `boolean`; \}; `private`: `boolean`; `pulls_url`: `string`; `pushed_at`: `string` \| `null`; `releases_url`: `string`; `size`: `number`; `squash_merge_commit_message?`: `"PR_BODY"` \| `"COMMIT_MESSAGES"` \| `"BLANK"`; `squash_merge_commit_title?`: `"PR_TITLE"` \| `"COMMIT_OR_PR_TITLE"`; `ssh_url`: `string`; `stargazers_count`: `number`; `stargazers_url`: `string`; `starred_at?`: `string`; `statuses_url`: `string`; `subscribers_url`: `string`; `subscription_url`: `string`; `svn_url`: `string`; `tags_url`: `string`; `teams_url`: `string`; `temp_clone_token?`: `string`; `topics?`: `string`[]; `trees_url`: `string`; `updated_at`: `string` \| `null`; `url`: `string`; `use_squash_pr_title_as_default?`: `boolean`; `visibility?`: `string`; `watchers`: `number`; `watchers_count`: `number`; `web_commit_signoff_required?`: `boolean`; \}; `sha`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}; `body`: `string` \| `null`; `changed_files`: `number`; `closed_at`: `string` \| `null`; `comments`: `number`; `comments_url`: `string`; `commits`: `number`; `commits_url`: `string`; `created_at`: `string`; `deletions`: `number`; `diff_url`: `string`; `draft?`: `boolean`; `head`: \{ `label`: `string`; `ref`: `string`; `repo`: \{ `allow_auto_merge?`: `boolean`; `allow_forking?`: `boolean`; `allow_merge_commit?`: `boolean`; `allow_rebase_merge?`: `boolean`; `allow_squash_merge?`: `boolean`; `allow_update_branch?`: `boolean`; `anonymous_access_enabled?`: `boolean`; `archive_url`: `string`; `archived`: `boolean`; `assignees_url`: `string`; `blobs_url`: `string`; `branches_url`: `string`; `clone_url`: `string`; `code_search_index_status?`: \{ `lexical_commit_sha?`: `string`; `lexical_search_ok?`: `boolean`; \}; `collaborators_url`: `string`; `comments_url`: `string`; `commits_url`: `string`; `compare_url`: `string`; `contents_url`: `string`; `contributors_url`: `string`; `created_at`: `string` \| `null`; `default_branch`: `string`; `delete_branch_on_merge?`: `boolean`; `deployments_url`: `string`; `description`: `string` \| `null`; `disabled`: `boolean`; `downloads_url`: `string`; `events_url`: `string`; `fork`: `boolean`; `forks`: `number`; `forks_count`: `number`; `forks_url`: `string`; `full_name`: `string`; `git_commits_url`: `string`; `git_refs_url`: `string`; `git_tags_url`: `string`; `git_url`: `string`; `has_discussions?`: `boolean`; `has_downloads`: `boolean`; `has_issues`: `boolean`; `has_pages`: `boolean`; `has_projects`: `boolean`; `has_wiki`: `boolean`; `homepage`: `string` \| `null`; `hooks_url`: `string`; `html_url`: `string`; `id`: `number`; `is_template?`: `boolean`; `issue_comment_url`: `string`; `issue_events_url`: `string`; `issues_url`: `string`; `keys_url`: `string`; `labels_url`: `string`; `language`: `string` \| `null`; `languages_url`: `string`; `license`: \{ `html_url?`: `string`; `key`: `string`; `name`: `string`; `node_id`: `string`; `spdx_id`: `string` \| `null`; `url`: `string` \| `null`; \} \| `null`; `master_branch?`: `string`; `merge_commit_message?`: `"PR_TITLE"` \| `"PR_BODY"` \| `"BLANK"`; `merge_commit_title?`: `"PR_TITLE"` \| `"MERGE_MESSAGE"`; `merges_url`: `string`; `milestones_url`: `string`; `mirror_url`: `string` \| `null`; `name`: `string`; `node_id`: `string`; `notifications_url`: `string`; `open_issues`: `number`; `open_issues_count`: `number`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; `permissions?`: \{ `admin`: `boolean`; `maintain?`: `boolean`; `pull`: `boolean`; `push`: `boolean`; `triage?`: `boolean`; \}; `private`: `boolean`; `pulls_url`: `string`; `pushed_at`: `string` \| `null`; `releases_url`: `string`; `size`: `number`; `squash_merge_commit_message?`: `"PR_BODY"` \| `"COMMIT_MESSAGES"` \| `"BLANK"`; `squash_merge_commit_title?`: `"PR_TITLE"` \| `"COMMIT_OR_PR_TITLE"`; `ssh_url`: `string`; `stargazers_count`: `number`; `stargazers_url`: `string`; `starred_at?`: `string`; `statuses_url`: `string`; `subscribers_url`: `string`; `subscription_url`: `string`; `svn_url`: `string`; `tags_url`: `string`; `teams_url`: `string`; `temp_clone_token?`: `string`; `topics?`: `string`[]; `trees_url`: `string`; `updated_at`: `string` \| `null`; `url`: `string`; `use_squash_pr_title_as_default?`: `boolean`; `visibility?`: `string`; `watchers`: `number`; `watchers_count`: `number`; `web_commit_signoff_required?`: `boolean`; \}; `sha`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}; `html_url`: `string`; `id`: `number`; `issue_url`: `string`; `labels`: `object`[]; `locked`: `boolean`; `maintainer_can_modify`: `boolean`; `merge_commit_sha`: `string` \| `null`; `mergeable`: `boolean` \| `null`; `mergeable_state`: `string`; `merged`: `boolean`; `merged_at`: `string` \| `null`; `merged_by`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `milestone`: \{ `closed_at`: `string` \| `null`; `closed_issues`: `number`; `created_at`: `string`; `creator`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; `description`: `string` \| `null`; `due_on`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `labels_url`: `string`; `node_id`: `string`; `number`: `number`; `open_issues`: `number`; `state`: `"open"` \| `"closed"`; `title`: `string`; `updated_at`: `string`; `url`: `string`; \} \| `null`; `node_id`: `string`; `number`: `number`; `patch_url`: `string`; `rebaseable?`: `boolean` \| `null`; `requested_reviewers?`: `object`[] \| `null`; `requested_teams?`: `object`[] \| `null`; `review_comment_url`: `string`; `review_comments`: `number`; `review_comments_url`: `string`; `state`: `"open"` \| `"closed"`; `statuses_url`: `string`; `title`: `string`; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}\>

***

### getPRFiles()

> **getPRFiles**(`prNumber`): `Promise`\<`object`[]\>

Defined in: [packages/agentic-control/src/github/client.ts:255](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L255)

Get files changed in a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<`object`[]\>

***

### getReviewComments()

> **getReviewComments**(`prNumber`): `Promise`\<`object`[]\>

Defined in: [packages/agentic-control/src/github/client.ts:385](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L385)

Get review comments on a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<`object`[]\>

***

### getReviews()

> **getReviews**(`prNumber`): `Promise`\<`object`[]\>

Defined in: [packages/agentic-control/src/github/client.ts:371](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L371)

Get reviews on a PR

#### Parameters

##### prNumber

`number`

#### Returns

`Promise`\<`object`[]\>

***

### mergePR()

> **mergePR**(`prNumber`, `method?`): `Promise`\<\{ `merged`: `boolean`; `message`: `string`; `sha`: `string`; \}\>

Defined in: [packages/agentic-control/src/github/client.ts:352](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L352)

Merge a pull request (instance version)

#### Parameters

##### prNumber

`number`

##### method?

`"merge"` | `"squash"` | `"rebase"`

#### Returns

`Promise`\<\{ `merged`: `boolean`; `message`: `string`; `sha`: `string`; \}\>

***

### postComment()

> **postComment**(`prNumber`, `body`): `Promise`\<\{ `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `body?`: `string`; `body_html?`: `string`; `body_text?`: `string`; `created_at`: `string`; `html_url`: `string`; `id`: `number`; `issue_url`: `string`; `node_id`: `string`; `performed_via_github_app?`: \{ `client_id?`: `string`; `created_at`: `string`; `description`: `string` \| `null`; `events`: `string`[]; `external_url`: `string`; `html_url`: `string`; `id`: `number`; `installations_count?`: `number`; `name`: `string`; `node_id`: `string`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| \{ `avatar_url`: `string`; `created_at`: `string` \| `null`; `description?`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `name`: `string`; `node_id`: `string`; `slug`: `string`; `updated_at`: `string` \| `null`; `website_url?`: `string` \| `null`; \}; `permissions`: \{\[`key`: `string`\]: `string` \| `undefined`; `checks?`: `string`; `contents?`: `string`; `deployments?`: `string`; `issues?`: `string`; `metadata?`: `string`; \}; `slug?`: `string`; `updated_at`: `string`; \} \| `null`; `reactions?`: \{ `-1`: `number`; `+1`: `number`; `confused`: `number`; `eyes`: `number`; `heart`: `number`; `hooray`: `number`; `laugh`: `number`; `rocket`: `number`; `total_count`: `number`; `url`: `string`; \}; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; \}\>

Defined in: [packages/agentic-control/src/github/client.ts:486](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L486)

Post a comment (instance version)

#### Parameters

##### prNumber

`number`

##### body

`string`

#### Returns

`Promise`\<\{ `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `body?`: `string`; `body_html?`: `string`; `body_text?`: `string`; `created_at`: `string`; `html_url`: `string`; `id`: `number`; `issue_url`: `string`; `node_id`: `string`; `performed_via_github_app?`: \{ `client_id?`: `string`; `created_at`: `string`; `description`: `string` \| `null`; `events`: `string`[]; `external_url`: `string`; `html_url`: `string`; `id`: `number`; `installations_count?`: `number`; `name`: `string`; `node_id`: `string`; `owner`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| \{ `avatar_url`: `string`; `created_at`: `string` \| `null`; `description?`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `name`: `string`; `node_id`: `string`; `slug`: `string`; `updated_at`: `string` \| `null`; `website_url?`: `string` \| `null`; \}; `permissions`: \{\[`key`: `string`\]: `string` \| `undefined`; `checks?`: `string`; `contents?`: `string`; `deployments?`: `string`; `issues?`: `string`; `metadata?`: `string`; \}; `slug?`: `string`; `updated_at`: `string`; \} \| `null`; `reactions?`: \{ `-1`: `number`; `+1`: `number`; `confused`: `number`; `eyes`: `number`; `heart`: `number`; `hooray`: `number`; `laugh`: `number`; `rocket`: `number`; `total_count`: `number`; `url`: `string`; \}; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \} \| `null`; \}\>

***

### replyToComment()

> **replyToComment**(`prNumber`, `commentId`, `body`): `Promise`\<\{ `_links`: \{ `html`: \{ `href`: `string`; \}; `pull_request`: \{ `href`: `string`; \}; `self`: \{ `href`: `string`; \}; \}; `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `body`: `string`; `body_html?`: `string`; `body_text?`: `string`; `commit_id`: `string`; `created_at`: `string`; `diff_hunk`: `string`; `html_url`: `string`; `id`: `number`; `in_reply_to_id?`: `number`; `line?`: `number`; `node_id`: `string`; `original_commit_id`: `string`; `original_line?`: `number`; `original_position?`: `number`; `original_start_line?`: `number` \| `null`; `path`: `string`; `position?`: `number`; `pull_request_review_id`: `number` \| `null`; `pull_request_url`: `string`; `reactions?`: \{ `-1`: `number`; `+1`: `number`; `confused`: `number`; `eyes`: `number`; `heart`: `number`; `hooray`: `number`; `laugh`: `number`; `rocket`: `number`; `total_count`: `number`; `url`: `string`; \}; `side?`: `"LEFT"` \| `"RIGHT"`; `start_line?`: `number` \| `null`; `start_side?`: `"LEFT"` \| `"RIGHT"` \| `null`; `subject_type?`: `"line"` \| `"file"`; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}\>

Defined in: [packages/agentic-control/src/github/client.ts:501](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L501)

Reply to a review comment

#### Parameters

##### prNumber

`number`

##### commentId

`number`

##### body

`string`

#### Returns

`Promise`\<\{ `_links`: \{ `html`: \{ `href`: `string`; \}; `pull_request`: \{ `href`: `string`; \}; `self`: \{ `href`: `string`; \}; \}; `author_association`: `"COLLABORATOR"` \| `"CONTRIBUTOR"` \| `"FIRST_TIMER"` \| `"FIRST_TIME_CONTRIBUTOR"` \| `"MANNEQUIN"` \| `"MEMBER"` \| `"NONE"` \| `"OWNER"`; `body`: `string`; `body_html?`: `string`; `body_text?`: `string`; `commit_id`: `string`; `created_at`: `string`; `diff_hunk`: `string`; `html_url`: `string`; `id`: `number`; `in_reply_to_id?`: `number`; `line?`: `number`; `node_id`: `string`; `original_commit_id`: `string`; `original_line?`: `number`; `original_position?`: `number`; `original_start_line?`: `number` \| `null`; `path`: `string`; `position?`: `number`; `pull_request_review_id`: `number` \| `null`; `pull_request_url`: `string`; `reactions?`: \{ `-1`: `number`; `+1`: `number`; `confused`: `number`; `eyes`: `number`; `heart`: `number`; `hooray`: `number`; `laugh`: `number`; `rocket`: `number`; `total_count`: `number`; `url`: `string`; \}; `side?`: `"LEFT"` \| `"RIGHT"`; `start_line?`: `number` \| `null`; `start_side?`: `"LEFT"` \| `"RIGHT"` \| `null`; `subject_type?`: `"line"` \| `"file"`; `updated_at`: `string`; `url`: `string`; `user`: \{ `avatar_url`: `string`; `email?`: `string` \| `null`; `events_url`: `string`; `followers_url`: `string`; `following_url`: `string`; `gists_url`: `string`; `gravatar_id`: `string` \| `null`; `html_url`: `string`; `id`: `number`; `login`: `string`; `name?`: `string` \| `null`; `node_id`: `string`; `organizations_url`: `string`; `received_events_url`: `string`; `repos_url`: `string`; `site_admin`: `boolean`; `starred_at?`: `string`; `starred_url`: `string`; `subscriptions_url`: `string`; `type`: `string`; `url`: `string`; `user_view_type?`: `string`; \}; \}\>

***

### createPR()

> `static` **createPR**(`owner`, `repo`, `options`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PullRequest`](/api/control/interfaces/pullrequest/)\>\>

Defined in: [packages/agentic-control/src/github/client.ts:269](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L269)

Create a pull request

#### Parameters

##### owner

`string`

##### repo

`string`

##### options

###### base

`string`

###### body?

`string`

###### draft?

`boolean`

###### head

`string`

###### title

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PullRequest`](/api/control/interfaces/pullrequest/)\>\>

***

### forPRReview()

> `static` **forPRReview**(): `Octokit` & `RestEndpointMethods` & `Api` & `object` \| `null`

Defined in: [packages/agentic-control/src/github/client.ts:97](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L97)

Get an Octokit instance for PR review operations.
Always uses the consistent PR review identity.

#### Returns

`Octokit` & `RestEndpointMethods` & `Api` & `object` \| `null`

***

### forRepo()

> `static` **forRepo**(`repoUrl`): `Octokit` & `RestEndpointMethods` & `Api` & `object` \| `null`

Defined in: [packages/agentic-control/src/github/client.ts:84](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L84)

Get an Octokit instance for a repository.
Automatically selects the correct token based on org.

#### Parameters

##### repoUrl

`string`

#### Returns

`Octokit` & `RestEndpointMethods` & `Api` & `object` \| `null`

***

### getPRStatic()

> `static` **getPRStatic**(`owner`, `repo`, `prNumber`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PullRequest`](/api/control/interfaces/pullrequest/)\>\>

Defined in: [packages/agentic-control/src/github/client.ts:207](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L207)

Get pull request information (static version)

#### Parameters

##### owner

`string`

##### repo

`string`

##### prNumber

`number`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PullRequest`](/api/control/interfaces/pullrequest/)\>\>

***

### getRepo()

> `static` **getRepo**(`owner`, `repo`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)\>\>

Defined in: [packages/agentic-control/src/github/client.ts:138](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L138)

Get repository information

#### Parameters

##### owner

`string`

##### repo

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)\>\>

***

### listOrgRepos()

> `static` **listOrgRepos**(`org`, `options?`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

Defined in: [packages/agentic-control/src/github/client.ts:165](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L165)

List repositories for an organization

#### Parameters

##### org

`string`

##### options?

###### perPage?

`number`

###### type?

`"all"` \| `"public"` \| `"private"` \| `"forks"` \| `"sources"` \| `"member"`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

***

### listPRComments()

> `static` **listPRComments**(`owner`, `repo`, `prNumber`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PRComment`](/api/control/interfaces/prcomment/)[]\>\>

Defined in: [packages/agentic-control/src/github/client.ts:413](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L413)

List PR comments (static version)

#### Parameters

##### owner

`string`

##### repo

`string`

##### prNumber

`number`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PRComment`](/api/control/interfaces/prcomment/)[]\>\>

***

### mergePRStatic()

> `static` **mergePRStatic**(`owner`, `repo`, `prNumber`, `options?`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

Defined in: [packages/agentic-control/src/github/client.ts:318](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L318)

Merge a pull request

#### Parameters

##### owner

`string`

##### repo

`string`

##### prNumber

`number`

##### options?

###### commitMessage?

`string`

###### commitTitle?

`string`

###### mergeMethod?

`"merge"` \| `"squash"` \| `"rebase"`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

***

### postPRComment()

> `static` **postPRComment**(`owner`, `repo`, `prNumber`, `body`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PRComment`](/api/control/interfaces/prcomment/)\>\>

Defined in: [packages/agentic-control/src/github/client.ts:449](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L449)

Post a PR comment (ALWAYS uses PR review identity)

#### Parameters

##### owner

`string`

##### repo

`string`

##### prNumber

`number`

##### body

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`PRComment`](/api/control/interfaces/prcomment/)\>\>

***

### requestReview()

> `static` **requestReview**(`owner`, `repo`, `prNumber`, `reviewers`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

Defined in: [packages/agentic-control/src/github/client.ts:517](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/github/client.ts#L517)

Request a review on a PR (ALWAYS uses PR review identity)

#### Parameters

##### owner

`string`

##### repo

`string`

##### prNumber

`number`

##### reviewers

`string`[]

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>
