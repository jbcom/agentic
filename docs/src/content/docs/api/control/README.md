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

- [ConfigErrorCode](/api/control/enumerations/configerrorcode/)
- [DockerErrorCode](/api/control/enumerations/dockererrorcode/)
- [SandboxErrorCode](/api/control/enumerations/sandboxerrorcode/)

## Classes

- [AIAnalyzer](/api/control/classes/aianalyzer/)
- [ConfigurationError](/api/control/classes/configurationerror/)
- [ContainerManager](/api/control/classes/containermanager/)
- [CrewTool](/api/control/classes/crewtool/)
- [CrewToolError](/api/control/classes/crewtoolerror/)
- [CursorAPI](/api/control/classes/cursorapi/)
- [DockerBuildError](/api/control/classes/dockerbuilderror/)
- [Fleet](/api/control/classes/fleet/)
- [GitHubClient](/api/control/classes/githubclient/)
- [HandoffManager](/api/control/classes/handoffmanager/)
- [SandboxError](/api/control/classes/sandboxerror/)
- [SandboxExecutor](/api/control/classes/sandboxexecutor/)

## Interfaces

- [Agent](/api/control/interfaces/agent/)
- [AgenticConfig](/api/control/interfaces/agenticconfig/)
- [AgentSource](/api/control/interfaces/agentsource/)
- [AgentTarget](/api/control/interfaces/agenttarget/)
- [AIAnalyzerOptions](/api/control/interfaces/aianalyzeroptions/)
- [AnalysisResult](/api/control/interfaces/analysisresult/)
- [Blocker](/api/control/interfaces/blocker/)
- [CodeReviewResult](/api/control/interfaces/codereviewresult/)
- [ContainerConfig](/api/control/interfaces/containerconfig/)
- [ContainerResult](/api/control/interfaces/containerresult/)
- [Conversation](/api/control/interfaces/conversation/)
- [ConversationMessage](/api/control/interfaces/conversationmessage/)
- [CoordinationConfig](/api/control/interfaces/coordinationconfig/)
- [CrewInfo](/api/control/interfaces/crewinfo/)
- [CrewListResponse](/api/control/interfaces/crewlistresponse/)
- [CrewResult](/api/control/interfaces/crewresult/)
- [CrewToolConfig](/api/control/interfaces/crewtoolconfig/)
- [CursorAPIOptions](/api/control/interfaces/cursorapioptions/)
- [DiamondConfig](/api/control/interfaces/diamondconfig/)
- [FleetConfig](/api/control/interfaces/fleetconfig/)
- [HandoffContext](/api/control/interfaces/handoffcontext/)
- [HandoffOptions](/api/control/interfaces/handoffoptions/)
- [HandoffResult](/api/control/interfaces/handoffresult/)
- [InvokeCrewOptions](/api/control/interfaces/invokecrewoptions/)
- [OrganizationConfig](/api/control/interfaces/organizationconfig/)
- [PRComment](/api/control/interfaces/prcomment/)
- [ProviderOptions](/api/control/interfaces/provideroptions/)
- [PullRequest](/api/control/interfaces/pullrequest/)
- [Repository](/api/control/interfaces/repository/)
- [Result](/api/control/interfaces/result/)
- [ReviewImprovement](/api/control/interfaces/reviewimprovement/)
- [ReviewIssue](/api/control/interfaces/reviewissue/)
- [SandboxOptions](/api/control/interfaces/sandboxoptions/)
- [SpawnContext](/api/control/interfaces/spawncontext/)
- [SpawnOptions](/api/control/interfaces/spawnoptions/)
- [TakeoverOptions](/api/control/interfaces/takeoveroptions/)
- [Task](/api/control/interfaces/task/)
- [TokenConfig](/api/control/interfaces/tokenconfig/)
- [TriageConfig](/api/control/interfaces/triageconfig/)
- [TriageResult](/api/control/interfaces/triageresult/)

## Type Aliases

- [AgentStatus](/api/control/type-aliases/agentstatus/)
- [AsyncResult](/api/control/type-aliases/asyncresult/)
- [CrewToolErrorCategory](/api/control/type-aliases/crewtoolerrorcategory/)
- [ModelFactory](/api/control/type-aliases/modelfactory/)
- [Priority](/api/control/type-aliases/priority/)
- [ReviewCategory](/api/control/type-aliases/reviewcategory/)
- [ReviewSeverity](/api/control/type-aliases/reviewseverity/)
- [SupportedProvider](/api/control/type-aliases/supportedprovider/)
- [TriageCategory](/api/control/type-aliases/triagecategory/)

## Variables

- [AgenticConfigSchema](/api/control/variables/agenticconfigschema/)
- [log](/api/control/variables/log/)
- [PROVIDER\_CONFIG](/api/control/variables/provider_config/)
- [safeConsole](/api/control/variables/safeconsole/)
- [VERSION](/api/control/variables/version/)

## Functions

- [addOrganization](/api/control/functions/addorganization/)
- [clearProviderCache](/api/control/functions/clearprovidercache/)
- [cloneRepo](/api/control/functions/clonerepo/)
- [createSafeError](/api/control/functions/createsafeerror/)
- [extractOrg](/api/control/functions/extractorg/)
- [getConfig](/api/control/functions/getconfig/)
- [getConfigPath](/api/control/functions/getconfigpath/)
- [getConfiguredOrgs](/api/control/functions/getconfiguredorgs/)
- [getConfigValue](/api/control/functions/getconfigvalue/)
- [getCursorApiKey](/api/control/functions/getcursorapikey/)
- [getDefaultApiKeyEnvVar](/api/control/functions/getdefaultapikeyenvvar/)
- [~~getDefaultModel~~](/api/control/functions/getdefaultmodel/)
- [getEnvForPRReview](/api/control/functions/getenvforprreview/)
- [getEnvForRepo](/api/control/functions/getenvforrepo/)
- [getFleetDefaults](/api/control/functions/getfleetdefaults/)
- [getLogLevel](/api/control/functions/getloglevel/)
- [getOrgConfig](/api/control/functions/getorgconfig/)
- [getOrLoadProvider](/api/control/functions/getorloadprovider/)
- [getPRReviewToken](/api/control/functions/getprreviewtoken/)
- [getPRReviewTokenEnvVar](/api/control/functions/getprreviewtokenenvvar/)
- [getSupportedProviders](/api/control/functions/getsupportedproviders/)
- [getTokenConfig](/api/control/functions/gettokenconfig/)
- [getTokenEnvVar](/api/control/functions/gettokenenvvar/)
- [getTokenForOrg](/api/control/functions/gettokenfororg/)
- [getTokenForRepo](/api/control/functions/gettokenforrepo/)
- [getTokenSummary](/api/control/functions/gettokensummary/)
- [getTriageApiKey](/api/control/functions/gettriageapikey/)
- [getTriageConfig](/api/control/functions/gettriageconfig/)
- [hasTokenForOrg](/api/control/functions/hastokenfororg/)
- [hasTokenForRepo](/api/control/functions/hastokenforrepo/)
- [initConfig](/api/control/functions/initconfig/)
- [isValidGitRef](/api/control/functions/isvalidgitref/)
- [isValidProvider](/api/control/functions/isvalidprovider/)
- [isValidRepoFormat](/api/control/functions/isvalidrepoformat/)
- [isVerbose](/api/control/functions/isverbose/)
- [loadConfigFromPath](/api/control/functions/loadconfigfrompath/)
- [loadProvider](/api/control/functions/loadprovider/)
- [resetConfig](/api/control/functions/resetconfig/)
- [resolveProviderOptions](/api/control/functions/resolveprovideroptions/)
- [safeDockerCommand](/api/control/functions/safedockercommand/)
- [safeGitCommand](/api/control/functions/safegitcommand/)
- [safeSpawn](/api/control/functions/safespawn/)
- [safeSpawnSync](/api/control/functions/safespawnsync/)
- [sanitizeEnvironment](/api/control/functions/sanitizeenvironment/)
- [sanitizeError](/api/control/functions/sanitizeerror/)
- [setConfig](/api/control/functions/setconfig/)
- [setTokenConfig](/api/control/functions/settokenconfig/)
- [validateCommandArgs](/api/control/functions/validatecommandargs/)
- [validateConfig](/api/control/functions/validateconfig/)
- [validateEnvVar](/api/control/functions/validateenvvar/)
- [validateEnvVarWithMessage](/api/control/functions/validateenvvarwithmessage/)
- [validateGitRef](/api/control/functions/validategitref/)
- [validatePositiveInt](/api/control/functions/validatepositiveint/)
- [validateRepository](/api/control/functions/validaterepository/)
- [validateTokens](/api/control/functions/validatetokens/)
