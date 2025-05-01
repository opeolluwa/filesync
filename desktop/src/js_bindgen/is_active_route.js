
export function is_active_route(target_route) {
  let current_route = window.location.pathname;

  return (
    current_route.toLowerCase().trim() === target_route.toLowerCase().trim()
  );
}
