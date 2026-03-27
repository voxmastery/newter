import React from 'react';
import SearchPalette from '@site/src/components/SearchPalette';
import '../theme/prism-newt';

export default function Root({children}) {
  return (
    <>
      {children}
      <SearchPalette />
    </>
  );
}
