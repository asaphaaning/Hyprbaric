import React from 'react';
import Link from '@docusaurus/Link';
import {useDoc} from '@docusaurus/plugin-content-docs/client';

function PageLink({direction, page}) {
  const isNext = direction === 'next';

  return (
    <Link
      className={`hyprDocPaginatorLink hyprDocPaginatorLink${isNext ? 'Next' : 'Previous'}`}
      to={page.permalink}
      aria-label={`${isNext ? 'Next' : 'Previous'}: ${page.title}`}>
      {!isNext && <span className="hyprDocPaginatorArrow" aria-hidden="true">‹</span>}
      <span className="hyprDocPaginatorText">
        <small>{isNext ? 'Next' : 'Previous'}</small>
        <strong>{page.title}</strong>
      </span>
      {isNext && <span className="hyprDocPaginatorArrow" aria-hidden="true">›</span>}
    </Link>
  );
}

export default function DocItemPaginator() {
  const {metadata} = useDoc();
  const {previous, next} = metadata;

  if (!previous && !next) {
    return null;
  }

  return (
    <nav className="hyprDocPaginator" aria-label="Documentation navigation">
      {previous && <PageLink direction="previous" page={previous} />}
      {next && <PageLink direction="next" page={next} />}
    </nav>
  );
}
