import React from 'react'
import { GrammarlyEditorPlugin } from '@grammarly/editor-sdk-react'

export default function GrammarlyEditor() {
    return (
        <GrammarlyEditorPlugin clientId="client_22ajvnNXjzUtbQddqHLtjC">
            <textarea />
        </GrammarlyEditorPlugin>
    )
}