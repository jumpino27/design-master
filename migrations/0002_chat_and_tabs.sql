-- Conversational Thread Messages & Multi-Tab Canvas Support

CREATE TABLE IF NOT EXISTS thread_messages (
    id TEXT PRIMARY KEY,
    thread_id TEXT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    attachments_json TEXT NOT NULL DEFAULT '[]',
    highlight_annotations_json TEXT NOT NULL DEFAULT '[]',
    quick_replies_json TEXT NOT NULL DEFAULT '[]',
    linked_run_id TEXT REFERENCES runs(id) ON DELETE SET NULL,
    target_tab_key TEXT,
    target_tab_label TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_thread_messages_thread_created ON thread_messages(thread_id, created_at ASC);
