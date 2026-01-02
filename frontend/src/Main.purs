module Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)
import Halogen as H
import Halogen.Aff as HA
import Halogen.HTML as HH
import Halogen.VDom.Driver (runUI)

-- | Main component
component :: forall q i o m. H.Component q i o m
component =
  H.mkComponent
    { initialState: identity
    , render
    , eval: H.mkEval H.defaultEval
    }
  where
  render _ =
    HH.div_
      [ HH.h1_ [ HH.text "Welcome to spire-board" ]
      , HH.p_ [ HH.text "PureScript + Halogen Frontend" ]
      ]

-- | Application entry point
main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody
  runUI component unit body
  H.liftEffect $ log "spire-board frontend started"
