import { createRootRoute, Link, Outlet } from '@tanstack/react-router';
import { FiSearch } from 'react-icons/fi';

export const Route = createRootRoute({
  component: () => (
    <div className="min-h-screen bg-gray-100">
      <header className="">
        <div className="max-w-7xl mx-auto py-3 pt-6 px-4 sm:px-6 lg:px-8 flex gap-4 justify-start items-center">
          <h1 className="text-base font-bold text-gray-900">enstate</h1>
          <nav className="">
            <ul className="flex space-x-4">
              <li>
                <Link 
                  to="/" 
                  className="text-blue-600 hover:text-blue-800 flex items-center gap-2"
                  activeProps={{
                    className: 'text-blue-800 font-bold',
                  }}
                >
                  <FiSearch className="w-4 h-4" /> Search
                </Link>
              </li>
              {/* Search link removed - functionality is now on homepage */}
            </ul>
          </nav>
        </div>
      </header>
      <main className="max-w-7xl mx-auto pb-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <Outlet />
        </div>
      </main>
    </div>
  ),
}); 