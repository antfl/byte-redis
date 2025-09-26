import { defineConfig, presetAttributify, transformerDirectives, presetWind3 } from 'unocss';

export default defineConfig({
  content: {
    pipeline: {
      include: [/\.(vue|ts|js|tsx|jsx)$/],
      exclude: ['node_modules/**', 'dist/**', '.git/**', '**/*.md'],
    },
  },
  presets: [presetWind3(), presetAttributify()],
  transformers: [transformerDirectives()],
});
