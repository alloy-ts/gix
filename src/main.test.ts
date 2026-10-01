import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { expect, test } from "vite-plus/test";
import { main } from "./main.ts";

test("main returns Hello, world!", () => {
  expect(main()).toBe("Hello, world!");
});

test("main initializes repo when path provided", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-ts-test-"));
  try {
    const res = main(tmpDir);
    expect(res).toContain("Initialized repo");
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
