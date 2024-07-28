// go bak to prev page
export function goToPrevPage() {
  window.history.back();
}
// navigate to the next page
export function goToNextPage() {
  window.history.forward();
}

/**
 * @function gotoPage - to to the specified page
 * @param routePath a string of the route path relative to the index route
 */

export interface AppRouterInterface {
  routePath: string;
}
export function goToPage({ routePath }: any) {
  if (typeof window !== "undefined") {
    // browser code
    window.location.href = routePath;
  }
}
