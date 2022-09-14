module Main exposing (..)

import Browser exposing (Document)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Decode as D
import Json.Encode as E
import Maybe exposing (withDefault)
import Time exposing (Posix, toHour, toMinute, toSecond, utc)


type Model
    = Loading
    | Failure
    | Success (List TournamentEvent)


type Msg
    = Loaded (Result Http.Error (List TournamentEvent))


main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Loading
    , Http.get
        { url = "https://netplay-bracket-finder.github.io/netplay-bracket-finder/events.json"
        , expect = Http.expectJson Loaded (D.list eventDecoder)
        }
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update (Loaded result) model =
    case result of
        Ok events ->
            ( Success events, Cmd.none )

        Err _ ->
            ( Failure, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


type alias TournamentEvent =
    { slug : String
    , tournament_name : String
    , event_name : String
    , image : String
    , entrants : Maybe Int
    , start_time : Int
    }


eventDecoder : D.Decoder TournamentEvent
eventDecoder =
    D.map6 TournamentEvent
        (D.field "slug" D.string)
        (D.field "tournament_name" D.string)
        (D.field "event_name" D.string)
        (D.field "image" D.string)
        (D.field "entrants" (D.nullable D.int))
        (D.field "start_time" D.int)


toUtcString : Time.Posix -> String
toUtcString time =
    String.fromInt (toHour utc time)
        ++ ":"
        ++ String.fromInt (toMinute utc time)
        ++ ":"
        ++ String.fromInt (toSecond utc time)
        ++ " (UTC)"

-- http://tachyons.io/components/collections/square-title-subtitle/index.html
tournamentDiv : TournamentEvent -> Html Msg
tournamentDiv event =
    article [ class "fl w-100 w-50-m w-25-ns pa2-ns" ]
        [ div [ class "aspect-ratio aspect-ratio--1x1" ]
            [ img [ style "background-image" (String.concat [ "url(", event.image, ")" ])
                  , class "db bg-center cover aspect-ratio--object" ] []
            ]
        , a [ class "ph2 ph0-ns pb3 link db"
            , href (String.concat [ "https://start.gg/", event.slug ])]
            [ h3 [ class "f5 f4-ns mb0 black-90" ] [ text event.event_name ]
            , h3 [ class "f6 f5 fw4 mt2 black-60" ] [ text (String.fromInt (withDefault 0 event.entrants)) ]
            , h3 [ class "f6 f5 fw4 mt2 black-60" ] [ text (toUtcString (Time.millisToPosix event.start_time)) ]
            ]
        ]

doc : Model -> List (Html Msg)
doc model =
    case model of
        Loading ->
            [ text "loading" ]

        Failure ->
            [ text "failed" ]

        Success events ->
            [ article []
                [ code []
                    [ h2 [ class "f3 fw4 pa3 mv0" ] [ text "Upcoming Melee Netplay Tournaments" ]
                    , h3 [ class "f5 fw4 pa3 mv0" ] [ text "Last Updated <TODO: UPDATE>" ]
                    ]
                , section [ class "cf w-100 pa2-ns" ] (List.map tournamentDiv events)
                ]
            ]


view : Model -> Document Msg
view model =
    { title = "next-bracket"
    , body = doc model
    }
