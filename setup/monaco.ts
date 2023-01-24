import { defineMonacoSetup } from '@slidev/types'

export default defineMonacoSetup(async (monaco) => {
  monaco.editor.setModelLanguage("rust");
})
