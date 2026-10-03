# @lib/gix

Node.js NAPI bindings for Git repository operations powered by [Gitoxide (`gix`)](https://github.com/GitoxideLabs/gitoxide).

## Features

- High performance native Node.js addon built with NAPI-RS and `gix`.
- Safety-focused design storing repository paths rather than raw pointers to prevent memory safety / Use-After-Free issues.
- Backwards compatible doc aliases mapping to `gix` / libgit2 conventions.
- Full TypeScript type definitions included.

## Installation

```bash
npm install @lib/gix
```

## Quick Start

```typescript
import { Repository, Signature } from '@lib/gix';

// Initialize a new repository
const repo = Repository.init('./my-repo');

// Create a blob
const blobId = repo.blob(Buffer.from('Hello Gitoxide!'));

// Get HEAD reference
const head = repo.head();
console.log('HEAD ref name:', head.name());
```

## Development

- Install dependencies:

```bash
npm install
```

- Build native addon:

```bash
npm run build
```

- Run unit tests:

```bash
npm test
```
