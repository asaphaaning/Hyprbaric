import React, {useEffect, useRef} from 'react';
import SearchBar from '@theme-original/SearchBar';

// The reference design puts a "results / N matches" header above the hits. The
// dropdown is built imperatively by autocomplete.js, so the header is kept in
// sync from a MutationObserver instead of being rendered by React.
export default function SearchBarWrapper(props) {
  const hostRef = useRef(null);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) {
      return undefined;
    }

    let frame = 0;

    const sync = () => {
      const menu = host.querySelector('[class*="dropdownMenu"]');
      if (!menu) {
        return;
      }
      const existing = menu.querySelector(':scope > .hyprSearchHeader');
      const count = menu.querySelectorAll('[role="option"]').length;
      if (count === 0) {
        existing?.remove();
        return;
      }
      const header = existing ?? document.createElement('div');
      if (!existing) {
        header.className = 'hyprSearchHeader';
        header.append(document.createElement('span'), document.createElement('span'));
        header.firstChild.textContent = 'results';
      }
      if (menu.firstChild !== header) {
        menu.prepend(header);
      }
      const label = `${count} ${count === 1 ? 'match' : 'matches'}`;
      if (header.lastChild.textContent !== label) {
        header.lastChild.textContent = label;
      }
    };

    const observer = new MutationObserver(() => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(sync);
    });
    observer.observe(host, {childList: true, subtree: true});

    // The field is drawn on the container, so its padding, the slash prefix and
    // the shortcut pill should focus the input like the input itself does.
    const focusInput = (event) => {
      const input = host.querySelector('.navbar__search-input');
      if (!input || event.target.closest('button, [role="option"], a')) {
        return;
      }
      if (event.target !== input) {
        event.preventDefault();
        input.focus();
      }
    };
    const field = host.querySelector('.navbar__search');
    field?.addEventListener('mousedown', focusInput);

    return () => {
      observer.disconnect();
      cancelAnimationFrame(frame);
      field?.removeEventListener('mousedown', focusInput);
    };
  }, []);

  return (
    <div className="hyprSearchBar" ref={hostRef}>
      <SearchBar {...props} />
    </div>
  );
}
