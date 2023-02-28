module.exports = {
    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaFeatures: {
            jsx: true,
        },
        ecmaVersion: 12,
        sourceType: "module",
    },
    plugins: ["@typescript-eslint", "react", "react-hooks"],
    ignorePatterns: "*_wasm_bg.js",
    rules: {
        "linebreak-style": ["error", "unix"],
        quotes: ["error", "double"],
        semi: ["error", "always"],
        "react/jsx-uses-react": "off",
        "react/react-in-jsx-scope": "off",
        "react/function-component-definition": [
            2,
            {
                unnamedComponents: "arrow-function",
                namedComponents: "arrow-function",
            },
        ],
        "@typescript-eslint/explicit-function-return-type": [
            0,
            {
                allowExpressions: false,
                allowTypedFunctionExpressions: false,
                allowHigherOrderFunctions: false,
                allowDirectConstAssertionInArrowFunctions: false,
                allowConciseArrowFunctionExpressionsStartingWithVoid: false,
            },
        ],
        "react-hooks/rules-of-hooks": "error",
        "react-hooks/exhaustive-deps": "error",
    },

    settings: {
        react: {
            version: "detect",
        },
    },
};
