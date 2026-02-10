import { Bench } from 'tinybench'

import { latexToAst } from '../index.js'


const b = new Bench()

b.add('Native frac 1/2', () => {
  latexToAst(String.raw`\frac{1}{2}`)
})

await b.run()

console.table(b.table())
