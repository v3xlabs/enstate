import { createRootRoute, Link, Outlet } from '@tanstack/react-router';

export const Route = createRootRoute({
  component: () => (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <h1 className="text-3xl font-bold text-gray-900">Search App</h1>
          <nav className="mt-4">
            <ul className="flex space-x-4">
              <li>
                <Link 
                  to="/" 
                  className="text-blue-600 hover:text-blue-800"
                  activeProps={{
                    className: 'text-blue-800 font-bold',
                  }}
                >
                  Home
                </Link>
              </li>
              {/* Search link removed - functionality is now on homepage */}
            </ul>
          </nav>
        </div>
      </header>
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <Outlet />
        </div>
      </main>
    </div>
  ),
}); 