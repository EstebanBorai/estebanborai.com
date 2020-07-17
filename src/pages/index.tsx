import React from "react"
import { Link } from "gatsby"

import Layout from "../components/layout"
import Image from "../components/image"
import SEO from "../components/seo"

const IndexPage = (): JSX.Element => (
  <Layout>
    <SEO title="Esteban Borai" />
    <h2>Page 1</h2>
    <Link to="page-2">Page 2</Link>
  </Layout>
)

export default IndexPage
