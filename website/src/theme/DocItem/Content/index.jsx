import React from 'react';
import clsx from 'clsx';
import {ThemeClassNames} from '@docusaurus/theme-common';
import {useDoc} from '@docusaurus/plugin-content-docs/client';
import MDXContent from '@theme/MDXContent';

export default function DocItemContent({children}) {
  const {metadata, frontMatter} = useDoc();
  const description = frontMatter.hide_lead ? null : frontMatter.description ?? metadata.description;

  return (
    <div className={clsx(ThemeClassNames.docs.docMarkdown, 'markdown', 'hyprDocMarkdown')}>
      <header className="hyprDocHeader">
        <p className="hyprDocKicker">{frontMatter.kicker ?? 'hyprbaric documentation'}</p>
        <h1>{metadata.title}</h1>
        {description && <p className="hyprDocLead">{description}</p>}
      </header>
      <MDXContent>{children}</MDXContent>
    </div>
  );
}
