import React from 'react';
import { GitHubContext } from 'context';

const LanguagesBars = (): JSX.Element => {
  const githubContext = React.useContext(GitHubContext);
  const topmost = 4;
  const langs = {  ​
  C: 1,
  ​
  "C#": 1,
  ​
  CSS: 2,
  ​
  Dart: 1,
  ​
  Dockerfile: 1,
  ​
  Go: 3,
  ​
  HTML: 2,
  ​
  JavaScript: 4,
  ​
  Python: 3,
  ​
  Rust: 1,
  ​
  Shell: 1,
  ​
  TSQL: 1,
  ​
  TypeScript: 3,
  ​
  "Vim script": 1,
};

  function equivalent(ref: number, value: number): number {
    return (value * 100) / ref;
  }

  return (
    <article>
      <h3>Project / Language Counter</h3>
    <ul className="top-languages">
      {
        langs && Object.keys(langs).map((lang) =>
          <li className="lang-item">
            <p 
              style={{
                width: `${equivalent(topmost, langs[lang])}%`
              }}
              className={lang.toLowerCase()}
            >
              {lang}: {langs[lang]}
            </p>
          </li>
        )
      }
    </ul>
    <small>Source: GitHub @estebanborai</small>
    </article>
  )
}

export default LanguagesBars;
