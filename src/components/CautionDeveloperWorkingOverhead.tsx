import React from 'react';

const CautionDeveloperWorkingOverhead = (): JSX.Element => (
  <section style={{
    alignItems: 'center',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'center',
    height: '80vh',
    width: '100vw'
  }}>
    <h2 style={{ fontSize: '2rem', textAlign: 'center' }}>Hi! And thanks for visiting me!</h2>
    <h3 style={{ fontSize: '2rem', margin: 0, textAlign: 'center' }}>👨‍💻🧉</h3>
    <em style={{ display: 'block', textAlign: 'center' }}>Im currently working on this project and as you can see... This project it's not done yet :( <br /> Feel free to come back later!</em>
    {/* I know, maybe I should make us of "styled-components"... */}
  </section>
);

export default CautionDeveloperWorkingOverhead;
