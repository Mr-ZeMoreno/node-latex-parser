import { latexToAst, latexToMathml, DisplayStyle } from "./index.js";

console.log(latexToAst(String.raw`\frac{1}{2}`))
console.log(latexToMathml(String.raw`\frac{1}{2}`, DisplayStyle.Block))