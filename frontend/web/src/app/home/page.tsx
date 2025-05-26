export default function HomePage() {
  return (
    <div className="p-8">
      <div className="mx-auto max-w-4xl space-y-8">
        <div className="rounded-lg border bg-card p-8 shadow-sm">
          <h1 className="mb-4 text-4xl font-bold text-foreground">
            Welcome to Gamer CM
          </h1>
          <p className="text-lg text-muted-foreground">
            Manage your gaming communities with ease.
          </p>
        </div>

        <div className="grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border bg-card p-6 shadow-sm">
            <h2 className="mb-2 text-2xl font-semibold text-foreground">
              Features
            </h2>
            <ul className="space-y-2 text-muted-foreground">
              <li>• Community Management</li>
              <li>• Team Organization</li>
              <li>• Player Tracking</li>
              <li>• Real-time Updates</li>
              <li>• Modern UI/UX</li>
            </ul>
          </div>

          <div className="rounded-lg border bg-card p-6 shadow-sm">
            <h2 className="mb-2 text-2xl font-semibold text-foreground">
              Quick Start
            </h2>
            <p className="text-muted-foreground">
              Select a community from the sidebar to get started.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
} 