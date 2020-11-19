import express from 'express'
import { ApolloServer, gql } from 'apollo-server-express'
import { QueryResolvers } from './__generated__/graphql'

const typeDefs = gql`
  type Query {
    hello: String
  }
`

const hello: QueryResolvers['hello'] = () => 'Hello world!'

const resolvers = {
  Query: {
    hello,
  },
}

const server = new ApolloServer({ typeDefs, resolvers })

const app = express()

server.applyMiddleware({ app })

app.listen({ port: 4000 }, () =>
  console.log(`🚀 Server ready at http://localhost:4000${server.graphqlPath}`)
)

