import { Link } from "gatsby"
import PropTypes from "prop-types"
import React from "react"
import './header.css';

const Header = (): JSX.Element => (
  <header id="header">
    <h1>Esteban Borai</h1>
  </header>
)

Header.propTypes = {
  siteTitle: PropTypes.string,
}

Header.defaultProps = {
  siteTitle: ``,
}

export default Header
