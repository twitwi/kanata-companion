 // @ts-expect-error missing types for s-expression
import sparse from 's-expression'

export type SExpression = Array<string | SExpression>
export const parse = (content: string): SExpression => sparse(content)
