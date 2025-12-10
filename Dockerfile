# Multi-stage build for optimization
FROM node:22-slim AS node-base
FROM python:3.13-slim AS python-base

FROM python-base AS final

# Install Node.js from node-base
COPY --from=node-base /usr/local/bin/node /usr/local/bin/
COPY --from=node-base /usr/local/lib/node_modules /usr/local/lib/node_modules
COPY --from=node-base /usr/local/bin/npm /usr/local/bin/
COPY --from=node-base /usr/local/bin/npx /usr/local/bin/

# Create symlinks for node
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm && \
    ln -s /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | \
    dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg && \
    chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | \
    tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && apt-get install -y gh && \
    rm -rf /var/lib/apt/lists/*

# Install Python package manager (uv)
RUN pip install uv

# Install pnpm
RUN npm install -g pnpm

# Create non-root user
RUN useradd -m -u 1000 agent
USER agent
WORKDIR /home/agent

# Install agentic-crew from PyPI
RUN pip install --user agentic-crew

# Install agentic-control from npm
RUN pnpm add -g agentic-control

# Set up environment
ENV PATH="/home/agent/.local/bin:${PATH}"

ENTRYPOINT ["agentic"]
CMD ["--help"]