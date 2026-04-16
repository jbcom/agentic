---
editUrl: false
next: false
prev: false
title: "@jbcom/agentic"
---

@jbcom/agentic

Orchestration layer for AI agent fleet management consuming @jbcom/agentic-triage primitives.

Features:
- Multi-agent orchestration (Ollama/Jules/Cursor routing)
- CI resolution and PR lifecycle pipelines
- GitHub Marketplace actions integration
- Intelligent token switching (auto-selects org-appropriate tokens)
- Fleet management (spawn, monitor, coordinate agents)
- AI-powered triage (conversation analysis, code review)
- Station-to-station handoff (agent continuity)
- Token-aware GitHub operations

## Enumerations

- [ConfigErrorCode](/api/agentic/enumerations/configerrorcode/)
- [DockerErrorCode](/api/agentic/enumerations/dockererrorcode/)
- [SandboxErrorCode](/api/agentic/enumerations/sandboxerrorcode/)

## Classes

- [AIAnalyzer](/api/agentic/classes/aianalyzer/)
- [ConfigurationError](/api/agentic/classes/configurationerror/)
- [ContainerManager](/api/agentic/classes/containermanager/)
- [CrewTool](/api/agentic/classes/crewtool/)
- [CrewToolError](/api/agentic/classes/crewtoolerror/)
- [CursorAPI](/api/agentic/classes/cursorapi/)
- [DockerBuildError](/api/agentic/classes/dockerbuilderror/)
- [Fleet](/api/agentic/classes/fleet/)
- [GitHubClient](/api/agentic/classes/githubclient/)
- [HandoffManager](/api/agentic/classes/handoffmanager/)
- [SandboxError](/api/agentic/classes/sandboxerror/)
- [SandboxExecutor](/api/agentic/classes/sandboxexecutor/)

## Interfaces

- [Agent](/api/agentic/interfaces/agent/)
- [AgenticConfig](/api/agentic/interfaces/agenticconfig/)
- [AgentSource](/api/agentic/interfaces/agentsource/)
- [AgentTarget](/api/agentic/interfaces/agenttarget/)
- [AIAnalyzerOptions](/api/agentic/interfaces/aianalyzeroptions/)
- [AnalysisResult](/api/agentic/interfaces/analysisresult/)
- [Blocker](/api/agentic/interfaces/blocker/)
- [CodeReviewResult](/api/agentic/interfaces/codereviewresult/)
- [ContainerConfig](/api/agentic/interfaces/containerconfig/)
- [ContainerResult](/api/agentic/interfaces/containerresult/)
- [Conversation](/api/agentic/interfaces/conversation/)
- [ConversationMessage](/api/agentic/interfaces/conversationmessage/)
- [CoordinationConfig](/api/agentic/interfaces/coordinationconfig/)
- [CrewInfo](/api/agentic/interfaces/crewinfo/)
- [CrewListResponse](/api/agentic/interfaces/crewlistresponse/)
- [CrewResult](/api/agentic/interfaces/crewresult/)
- [CrewToolConfig](/api/agentic/interfaces/crewtoolconfig/)
- [CursorAPIOptions](/api/agentic/interfaces/cursorapioptions/)
- [DiamondConfig](/api/agentic/interfaces/diamondconfig/)
- [FleetConfig](/api/agentic/interfaces/fleetconfig/)
- [HandoffContext](/api/agentic/interfaces/handoffcontext/)
- [HandoffOptions](/api/agentic/interfaces/handoffoptions/)
- [HandoffResult](/api/agentic/interfaces/handoffresult/)
- [InvokeCrewOptions](/api/agentic/interfaces/invokecrewoptions/)
- [OrganizationConfig](/api/agentic/interfaces/organizationconfig/)
- [PRComment](/api/agentic/interfaces/prcomment/)
- [ProviderOptions](/api/agentic/interfaces/provideroptions/)
- [PullRequest](/api/agentic/interfaces/pullrequest/)
- [Repository](/api/agentic/interfaces/repository/)
- [Result](/api/agentic/interfaces/result/)
- [ReviewImprovement](/api/agentic/interfaces/reviewimprovement/)
- [ReviewIssue](/api/agentic/interfaces/reviewissue/)
- [SandboxOptions](/api/agentic/interfaces/sandboxoptions/)
- [SpawnContext](/api/agentic/interfaces/spawncontext/)
- [SpawnOptions](/api/agentic/interfaces/spawnoptions/)
- [TakeoverOptions](/api/agentic/interfaces/takeoveroptions/)
- [Task](/api/agentic/interfaces/task/)
- [TokenConfig](/api/agentic/interfaces/tokenconfig/)
- [TriageConfig](/api/agentic/interfaces/triageconfig/)
- [TriageResult](/api/agentic/interfaces/triageresult/)

## Type Aliases

- [AgentStatus](/api/agentic/type-aliases/agentstatus/)
- [AsyncResult](/api/agentic/type-aliases/asyncresult/)
- [CrewToolErrorCategory](/api/agentic/type-aliases/crewtoolerrorcategory/)
- [ModelFactory](/api/agentic/type-aliases/modelfactory/)
- [Priority](/api/agentic/type-aliases/priority/)
- [ReviewCategory](/api/agentic/type-aliases/reviewcategory/)
- [ReviewSeverity](/api/agentic/type-aliases/reviewseverity/)
- [SupportedProvider](/api/agentic/type-aliases/supportedprovider/)
- [TriageCategory](/api/agentic/type-aliases/triagecategory/)

## Variables

- [AgenticConfigSchema](/api/agentic/variables/agenticconfigschema/)
- [log](/api/agentic/variables/log/)
- [PROVIDER\_CONFIG](/api/agentic/variables/provider_config/)
- [safeConsole](/api/agentic/variables/safeconsole/)
- [VERSION](/api/agentic/variables/version/)

## Functions

- [addOrganization](/api/agentic/functions/addorganization/)
- [clearProviderCache](/api/agentic/functions/clearprovidercache/)
- [cloneRepo](/api/agentic/functions/clonerepo/)
- [createSafeError](/api/agentic/functions/createsafeerror/)
- [extractOrg](/api/agentic/functions/extractorg/)
- [getConfig](/api/agentic/functions/getconfig/)
- [getConfigPath](/api/agentic/functions/getconfigpath/)
- [getConfiguredOrgs](/api/agentic/functions/getconfiguredorgs/)
- [getConfigValue](/api/agentic/functions/getconfigvalue/)
- [getCursorApiKey](/api/agentic/functions/getcursorapikey/)
- [getDefaultApiKeyEnvVar](/api/agentic/functions/getdefaultapikeyenvvar/)
- [~~getDefaultModel~~](/api/agentic/functions/getdefaultmodel/)
- [getEnvForPRReview](/api/agentic/functions/getenvforprreview/)
- [getEnvForRepo](/api/agentic/functions/getenvforrepo/)
- [getFleetDefaults](/api/agentic/functions/getfleetdefaults/)
- [getLogLevel](/api/agentic/functions/getloglevel/)
- [getOrgConfig](/api/agentic/functions/getorgconfig/)
- [getOrLoadProvider](/api/agentic/functions/getorloadprovider/)
- [getPRReviewToken](/api/agentic/functions/getprreviewtoken/)
- [getPRReviewTokenEnvVar](/api/agentic/functions/getprreviewtokenenvvar/)
- [getSupportedProviders](/api/agentic/functions/getsupportedproviders/)
- [getTokenConfig](/api/agentic/functions/gettokenconfig/)
- [getTokenEnvVar](/api/agentic/functions/gettokenenvvar/)
- [getTokenForOrg](/api/agentic/functions/gettokenfororg/)
- [getTokenForRepo](/api/agentic/functions/gettokenforrepo/)
- [getTokenSummary](/api/agentic/functions/gettokensummary/)
- [getTriageApiKey](/api/agentic/functions/gettriageapikey/)
- [getTriageConfig](/api/agentic/functions/gettriageconfig/)
- [hasTokenForOrg](/api/agentic/functions/hastokenfororg/)
- [hasTokenForRepo](/api/agentic/functions/hastokenforrepo/)
- [initConfig](/api/agentic/functions/initconfig/)
- [isValidGitRef](/api/agentic/functions/isvalidgitref/)
- [isValidProvider](/api/agentic/functions/isvalidprovider/)
- [isValidRepoFormat](/api/agentic/functions/isvalidrepoformat/)
- [isVerbose](/api/agentic/functions/isverbose/)
- [loadConfigFromPath](/api/agentic/functions/loadconfigfrompath/)
- [loadProvider](/api/agentic/functions/loadprovider/)
- [resetConfig](/api/agentic/functions/resetconfig/)
- [resolveProviderOptions](/api/agentic/functions/resolveprovideroptions/)
- [safeDockerCommand](/api/agentic/functions/safedockercommand/)
- [safeGitCommand](/api/agentic/functions/safegitcommand/)
- [safeSpawn](/api/agentic/functions/safespawn/)
- [safeSpawnSync](/api/agentic/functions/safespawnsync/)
- [sanitizeEnvironment](/api/agentic/functions/sanitizeenvironment/)
- [sanitizeError](/api/agentic/functions/sanitizeerror/)
- [setConfig](/api/agentic/functions/setconfig/)
- [setTokenConfig](/api/agentic/functions/settokenconfig/)
- [validateCommandArgs](/api/agentic/functions/validatecommandargs/)
- [validateConfig](/api/agentic/functions/validateconfig/)
- [validateEnvVar](/api/agentic/functions/validateenvvar/)
- [validateEnvVarWithMessage](/api/agentic/functions/validateenvvarwithmessage/)
- [validateGitRef](/api/agentic/functions/validategitref/)
- [validatePositiveInt](/api/agentic/functions/validatepositiveint/)
- [validateRepository](/api/agentic/functions/validaterepository/)
- [validateTokens](/api/agentic/functions/validatetokens/)
