import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const native = require("../index.js");

export const Repository = native.Repository;
export default native;
