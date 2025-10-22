// Centralized version export so UI and runtime can read project version at build time
// Vite supports importing JSON files when `resolveJsonModule` is enabled in tsconfig.
import pkg from '../package.json';

export const VERSION: string = (pkg && pkg.version) || '0.0.0';

export default VERSION;
