import React from 'react';
import clsx from 'clsx';
import {useWindowSize} from '@docusaurus/theme-common';
import {useDoc} from '@docusaurus/plugin-content-docs/client';
import ContentVisibility from '@theme/ContentVisibility';
import DocItemContent from '@theme/DocItem/Content';
import DocItemPaginator from '@theme/DocItem/Paginator';
import DocItemTOCDesktop from '@theme/DocItem/TOC/Desktop';
import DocItemTOCMobile from '@theme/DocItem/TOC/Mobile';
import DocVersionBanner from '@theme/DocVersionBanner';

function useDocTOC() {
  const {frontMatter, toc} = useDoc();
  const windowSize = useWindowSize();
  const hidden = frontMatter.hide_table_of_contents;
  const canRender = !hidden && toc.length > 0;

  return {
    hidden,
    mobile: canRender ? <DocItemTOCMobile /> : undefined,
    desktop: canRender && (windowSize === 'desktop' || windowSize === 'ssr') ? <DocItemTOCDesktop /> : undefined,
  };
}

export default function DocItemLayout({children}) {
  const docTOC = useDocTOC();
  const {metadata} = useDoc();

  return (
    <div className="hyprDocLayout">
      <div className={clsx('hyprDocArticleColumn', !docTOC.hidden && 'hyprDocArticleColumnWithToc')}>
        <ContentVisibility metadata={metadata} />
        <DocVersionBanner />
        <article className="hyprDocArticle">
          {docTOC.mobile}
          <DocItemContent>{children}</DocItemContent>
        </article>
        <DocItemPaginator />
      </div>
      {docTOC.desktop && <aside className="hyprDocToc">{docTOC.desktop}</aside>}
    </div>
  );
}
