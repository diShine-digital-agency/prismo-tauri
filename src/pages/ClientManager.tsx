import { useState, useEffect, useCallback } from "react";

interface Client {
  name: string;
  domain: string;
  industry: string;
  stack: string;
  analytics: string;
  notes: string;
  competitors: string[];
}

const defaultClient: Client = {
  name: "",
  domain: "",
  industry: "",
  stack: "",
  analytics: "",
  notes: "",
  competitors: [],
};

const sampleClients: Client[] = [
  {
    name: "Acme Corp",
    domain: "acme.com",
    industry: "E-commerce",
    stack: "WordPress + WooCommerce",
    analytics: "GA4 + GTM",
    notes: "Migration to headless CMS planned for Q3 2026",
    competitors: ["competitor1.com", "competitor2.com"],
  },
];

const STORAGE_KEY = "prismo_clients";

function loadClients(): Client[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw);
  } catch {
    // Ignore corrupt data, fall back to samples
  }
  return sampleClients;
}

function saveClients(clients: Client[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(clients));
}

export default function ClientManager() {
  const [clients, setClients] = useState<Client[]>(loadClients);
  const [editingClient, setEditingClient] = useState<Client | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [deleteConfirm, setDeleteConfirm] = useState<string | null>(null);

  // Persist clients whenever the list changes
  useEffect(() => {
    saveClients(clients);
  }, [clients]);

  const handleSave = useCallback(() => {
    if (!editingClient) return;
    if (!editingClient.name.trim() || !editingClient.domain.trim()) return;

    if (isCreating) {
      // Prevent duplicate domains
      if (clients.some((c) => c.domain === editingClient.domain)) {
        return;
      }
      setClients([...clients, editingClient]);
    } else {
      setClients(
        clients.map((c) => (c.domain === editingClient.domain ? editingClient : c))
      );
    }
    setEditingClient(null);
    setIsCreating(false);
  }, [editingClient, isCreating, clients]);

  const handleDelete = useCallback((domain: string) => {
    setClients((prev) => prev.filter((c) => c.domain !== domain));
    setDeleteConfirm(null);
  }, []);

  const filteredClients = clients.filter((c) => {
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return (
      c.name.toLowerCase().includes(q) ||
      c.domain.toLowerCase().includes(q) ||
      c.industry.toLowerCase().includes(q)
    );
  });

  if (editingClient) {
    return (
      <div className="p-8 max-w-2xl mx-auto">
        <button
          onClick={() => { setEditingClient(null); setIsCreating(false); }}
          className="text-sm text-gray-400 hover:text-gray-200 mb-6 flex items-center gap-1"
        >
          ← Back to clients
        </button>

        <h1 className="text-2xl font-bold text-white mb-6">
          {isCreating ? "New Client" : "Edit Client"}
        </h1>

        <div className="bg-gray-900 border border-gray-800 rounded-xl p-6 space-y-4">
          {[
            { label: "Client Name", key: "name" as const, placeholder: "Acme Corp", id: "client-name", required: true },
            { label: "Domain", key: "domain" as const, placeholder: "acme.com", id: "client-domain", required: true },
            { label: "Industry", key: "industry" as const, placeholder: "E-commerce", id: "client-industry", required: false },
            { label: "Tech Stack", key: "stack" as const, placeholder: "WordPress + WooCommerce", id: "client-stack", required: false },
            { label: "Analytics", key: "analytics" as const, placeholder: "GA4 + GTM", id: "client-analytics", required: false },
          ].map((field) => (
            <div key={field.key}>
              <label htmlFor={field.id} className="block text-sm text-gray-400 mb-1.5">
                {field.label}{field.required && <span className="text-red-400 ml-1">*</span>}
              </label>
              <input
                id={field.id}
                type="text"
                value={editingClient[field.key]}
                onChange={(e) =>
                  setEditingClient({ ...editingClient, [field.key]: e.target.value })
                }
                placeholder={field.placeholder}
                disabled={!isCreating && field.key === "domain"}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-gray-200 placeholder-gray-600 focus:outline-none focus:border-purple-500 disabled:opacity-50"
              />
            </div>
          ))}

          <div>
            <label htmlFor="client-notes" className="block text-sm text-gray-400 mb-1.5">Notes</label>
            <textarea
              id="client-notes"
              value={editingClient.notes}
              onChange={(e) =>
                setEditingClient({ ...editingClient, notes: e.target.value })
              }
              placeholder="Additional context for audits..."
              rows={3}
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-gray-200 placeholder-gray-600 focus:outline-none focus:border-purple-500 resize-none"
            />
          </div>

          <div>
            <label htmlFor="client-competitors" className="block text-sm text-gray-400 mb-1.5">
              Competitors (comma-separated)
            </label>
            <input
              id="client-competitors"
              type="text"
              value={editingClient.competitors.join(", ")}
              onChange={(e) =>
                setEditingClient({
                  ...editingClient,
                  competitors: e.target.value.split(",").map((s) => s.trim()).filter(Boolean),
                })
              }
              placeholder="competitor1.com, competitor2.com"
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-gray-200 placeholder-gray-600 focus:outline-none focus:border-purple-500"
            />
          </div>

          <div className="flex gap-3 pt-2">
            <button
              onClick={handleSave}
              disabled={!editingClient.name.trim() || !editingClient.domain.trim()}
              className="bg-purple-600 hover:bg-purple-700 text-white font-medium px-6 py-2.5 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Save Client
            </button>
            <button
              onClick={() => { setEditingClient(null); setIsCreating(false); }}
              className="bg-gray-800 hover:bg-gray-700 text-gray-300 px-6 py-2.5 rounded-lg transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-4xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-white mb-2">Client Profiles</h1>
          <p className="text-gray-400">
            Manage client context for more relevant audit results.
          </p>
        </div>
        <button
          onClick={() => { setEditingClient({ ...defaultClient }); setIsCreating(true); }}
          className="bg-purple-600 hover:bg-purple-700 text-white font-medium px-4 py-2.5 rounded-lg transition-colors flex items-center gap-2"
        >
          + New Client
        </button>
      </div>

      {/* Search */}
      {clients.length > 0 && (
        <div className="mb-4">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search clients by name, domain, or industry…"
            aria-label="Search clients"
            className="w-full bg-gray-900 border border-gray-800 rounded-lg px-4 py-2.5 text-gray-200 placeholder-gray-600 focus:outline-none focus:border-purple-500"
          />
        </div>
      )}

      <div className="space-y-3">
        {filteredClients.map((client) => (
          <div
            key={client.domain}
            className="bg-gray-900 border border-gray-800 rounded-xl p-5"
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start gap-4">
                <div className="w-12 h-12 rounded-xl bg-purple-600/20 border border-purple-500/30 flex items-center justify-center text-xl" aria-hidden="true">
                  👥
                </div>
                <div>
                  <h3 className="font-semibold text-gray-200">{client.name}</h3>
                  <p className="text-sm text-purple-400">{client.domain}</p>
                  <div className="flex flex-wrap gap-2 mt-2">
                    {client.industry && (
                      <span className="text-xs bg-gray-800 text-gray-400 px-2 py-0.5 rounded">
                        {client.industry}
                      </span>
                    )}
                    {client.stack && (
                      <span className="text-xs bg-gray-800 text-gray-400 px-2 py-0.5 rounded">
                        {client.stack}
                      </span>
                    )}
                    {client.analytics && (
                      <span className="text-xs bg-gray-800 text-gray-400 px-2 py-0.5 rounded">
                        {client.analytics}
                      </span>
                    )}
                  </div>
                  {client.notes && (
                    <p className="text-xs text-gray-500 mt-2">{client.notes}</p>
                  )}
                </div>
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setEditingClient({ ...client })}
                  aria-label={`Edit ${client.name}`}
                  className="text-sm text-gray-400 hover:text-purple-400 transition-colors px-3 py-1.5 rounded-lg hover:bg-gray-800"
                >
                  Edit
                </button>
                {deleteConfirm === client.domain ? (
                  <div className="flex items-center gap-1">
                    <button
                      onClick={() => handleDelete(client.domain)}
                      aria-label={`Confirm delete ${client.name}`}
                      className="text-sm text-red-400 hover:text-red-300 transition-colors px-2 py-1.5 rounded-lg hover:bg-red-900/30"
                    >
                      Confirm
                    </button>
                    <button
                      onClick={() => setDeleteConfirm(null)}
                      aria-label="Cancel delete"
                      className="text-sm text-gray-400 hover:text-gray-200 transition-colors px-2 py-1.5 rounded-lg hover:bg-gray-800"
                    >
                      Cancel
                    </button>
                  </div>
                ) : (
                  <button
                    onClick={() => setDeleteConfirm(client.domain)}
                    aria-label={`Delete ${client.name}`}
                    className="text-sm text-gray-400 hover:text-red-400 transition-colors px-3 py-1.5 rounded-lg hover:bg-gray-800"
                  >
                    Delete
                  </button>
                )}
              </div>
            </div>
          </div>
        ))}

        {filteredClients.length === 0 && clients.length > 0 && (
          <div className="text-center py-12 text-gray-500">
            <p>No clients match your search.</p>
          </div>
        )}

        {clients.length === 0 && (
          <div className="text-center py-12 text-gray-500">
            <p className="text-4xl mb-3" aria-hidden="true">👥</p>
            <p>No clients yet. Add your first client profile.</p>
          </div>
        )}
      </div>
    </div>
  );
}
