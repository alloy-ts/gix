import console from "node:console";
import { Repository } from "./lib.ts";

export const main = (repoPath?: string) => {
  if (repoPath) {
    const repo = Repository.init(repoPath);
    return `Initialized repo at ${repo.path()}`;
  }
  return "Hello, world!";
};

console.log(main());
