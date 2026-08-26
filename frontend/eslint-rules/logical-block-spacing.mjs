const NEWLINE = '\n';
const EMPTY_LINE = `${NEWLINE}${NEWLINE}`;

const declarationTypes = new Set([
  'ClassDeclaration',
  'FunctionDeclaration',
  'TSEnumDeclaration',
  'TSInterfaceDeclaration',
  'TSTypeAliasDeclaration',
  'VariableDeclaration',
]);

const isFunctionValue = (node) =>
  node.type === 'VariableDeclaration' &&
  node.declarations.every(
    ({ init }) =>
      init?.type === 'ArrowFunctionExpression' ||
      init?.type === 'FunctionExpression',
  );

const unwrapExport = (node) =>
  node.type === 'ExportNamedDeclaration' || node.type === 'ExportDefaultDeclaration'
    ? node.declaration
    : node;

const getBlockStatementKind = (node) => {
  if (node.type === 'ReturnStatement') {
    return 'return';
  }

  if (isFunctionValue(node) || node.type === 'FunctionDeclaration') {
    return 'handler';
  }

  if (node.type === 'VariableDeclaration') {
    return 'declaration';
  }

  return 'other';
};

const hasCommentBetween = (sourceCode, previous, current) =>
  sourceCode
    .getAllComments()
    .some(
      (comment) =>
        comment.range[0] >= previous.range[1] && comment.range[1] <= current.range[0],
    );

const hasExactlyOneEmptyLine = (sourceCode, previous, current) =>
  /^\n[\t ]*\n[\t ]*$/.test(
    sourceCode.text.slice(previous.range[1], current.range[0]),
  );

const getIndentation = (sourceCode, node) => {
  const lineStart = sourceCode.text.lastIndexOf(NEWLINE, node.range[0] - 1) + 1;

  return sourceCode.text.slice(lineStart, node.range[0]);
};

const requiresEmptyLine = (previous, current) => {
  const previousKind = getBlockStatementKind(previous);
  const currentKind = getBlockStatementKind(current);

  if (previousKind === 'declaration' && currentKind !== 'declaration') {
    return true;
  }

  if (previousKind === 'handler' && currentKind !== 'handler') {
    return true;
  }

  return currentKind === 'return' && previousKind !== 'return';
};

const reportSpacing = (context, previous, current) => {
  const sourceCode = context.sourceCode;

  if (hasExactlyOneEmptyLine(sourceCode, previous, current)) {
    return;
  }

  const canFix = !hasCommentBetween(sourceCode, previous, current);

  context.report({
    node: current,
    messageId: 'expectedEmptyLine',
    fix: canFix
      ? (fixer) =>
          fixer.replaceTextRange(
            [previous.range[1], current.range[0]],
            `${EMPTY_LINE}${getIndentation(sourceCode, current)}`,
          )
      : null,
  });
};

export default {
  rules: {
    'logical-block-spacing': {
      meta: {
        type: 'layout',
        docs: {
          description: 'enforce empty lines between TypeScript logical blocks',
        },
        fixable: 'whitespace',
        schema: [],
        messages: {
          expectedEmptyLine: 'Expected exactly one empty line between logical blocks.',
        },
      },
      create(context) {
        const checkTopLevelDeclarations = (body) => {
          let previousDeclaration = null;

          for (const statement of body) {
            const declaration = unwrapExport(statement);
            if (!declarationTypes.has(declaration?.type)) {
              previousDeclaration = null;
              continue;
            }

            if (previousDeclaration) {
              reportSpacing(context, previousDeclaration, statement);
            }

            previousDeclaration = statement;
          }
        };

        const checkBlockStatements = (body) => {
          for (let index = 1; index < body.length; index += 1) {
            const previous = body[index - 1];
            const current = body[index];

            if (requiresEmptyLine(previous, current)) {
              reportSpacing(context, previous, current);
            }
          }
        };

        return {
          Program(node) {
            checkTopLevelDeclarations(node.body);
          },
          BlockStatement(node) {
            checkBlockStatements(node.body);
          },
        };
      },
    },
  },
};
