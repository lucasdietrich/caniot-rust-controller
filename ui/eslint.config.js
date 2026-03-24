import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";

export default [
  { ignores: ["dist"] },
  js.configs.recommended,
  // TypeScript source files
  {
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
      parser: tsParser,
    },
    plugins: {
      "@typescript-eslint": tsPlugin,
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
    },
    rules: {
      ...tsPlugin.configs.recommended.rules,
      // Classic react-hooks rules only (v7 recommended also includes React
      // Compiler rules which require opting in to the React Compiler)
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "warn",
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
      // Disable base rule — @typescript-eslint/no-unused-vars handles this
      "no-unused-vars": "off",
      // Disable base rule — TypeScript handles this
      "no-undef": "off",
      // Downgrade to warnings — pre-existing code quality issues
      // Allow _-prefixed names as intentionally unused
      "@typescript-eslint/no-unused-vars": ["warn", {
        "argsIgnorePattern": "^_",
        "varsIgnorePattern": "^_",
        "caughtErrorsIgnorePattern": "^_",
      }],
      "@typescript-eslint/no-explicit-any": "warn",
      // Disable overly strict empty-object type rule
      "@typescript-eslint/no-empty-object-type": "warn",
    },
  },
  // Vite config and other Node.js config files
  {
    files: ["vite.config.ts", "eslint.config.js"],
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
    },
  },
];

