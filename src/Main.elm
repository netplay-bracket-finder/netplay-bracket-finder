module Main exposing (..)

import Browser exposing (Document)
import Html exposing (..)
import Html.Attributes exposing (..)



-- import Html.Events exposing (onInput, onClick)
-- import Http
-- import Json.Decode as D
-- import Json.Encode as E


main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type Model
    = Init (List TournamentEvent)


type alias TournamentEvent =
    { name : String
    , image : String
    , entrants : Int
    , startTime : String
    }


placeHolder : TournamentEvent
placeHolder =
    { name = "Legs' Le Tournament #41"
    , image = "https://images.smash.gg/images/tournament/460848/image-4f3871030bbb1e419191348fe7552839.png"
    , entrants = 27
    , startTime = "Sat Jul 16 5pm"
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Init (List.repeat 25 placeHolder), Cmd.none )


type Msg
    = Null


update : Msg -> Model -> ( Model, Cmd Msg )
update Null model =
    ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


tournamentDiv : TournamentEvent -> Html Msg
tournamentDiv event =
    div [ class "fl w-50 w-25-m w-20-l pa2" ]
        [ img [ src event.image, class "db link dim tc" ] []
        , dl [ class "m52 f6 lh-copy" ]
            [ dd [ class "ml0 black truncate w-100" ] [ text event.name ]
            , dd [ class "m10 gray truncate w-100" ] [ text (String.fromInt event.entrants) ]
            , dd [ class "ml0 gray truncate w-100" ] [ text event.startTime ]
            ]
        ]


doc : Model -> List (Html Msg)
doc (Init events) =
    [ article []
        [ code []
            [ h2 [ class "f3 fw4 pa3 mv0" ] [ text "Upcoming Melee Netplay Tournaments" ]
            , h3 [ class "f5 fw4 pa3 mv0" ] [ text "Last Updated <TODO: UPDATE>" ]
            ]
        , div [ class "cf pa2" ] (List.map tournamentDiv events)
        ]
    ]


view : Model -> Document Msg
view model =
    { title = "next-bracket"
    , body = doc model
    }
