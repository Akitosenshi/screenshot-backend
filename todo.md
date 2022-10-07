## Screenshot service

Der eigens gehostete raspberry pi wird folgens als Server bezeichnet

- sollte screenshots verarbeiten von einem eigenen server
- screenshots sollten zum service via rest crud/anderer api an Server gesendet werden

## Stories
- Server muss gesendeten screenshot speichern
- Screenshots sollten via url aufrufbar sein
- Server muss gegen unautorisierten zugriff geschützt sein (JWT , z.b.) / basic auth: "Header: {"Basic-Auth("Username:"admin", Password:"admin"")"}" / "Headers: "Authorization: Bearer: {eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJleHAiOjk5ODE5MTkxOSwicGVyc29uYWxfc2NvcGUiOjF9.V7qpdb5h_c54-IH90DQ2VDh8_-vIXMLtM97cugQ7B8w}"
- Hochgeladene screenshots sollen vom server benannt werden (uuid muss generiert werden)
- Screenshot muss als url an den client zurueckgegeben werden (durch response von curl/jq geparsed und weitergegeben über clipboard programm)

### Fragen
- Wie speichert der Server den screenshot
    - via rest api call & file write
- Wie wird der server darüber benachrichtigt über einen neuen screenshot?
    - REST aufruf
- Wie wird url ausgeliefert (generiert durch wen?) /wie wird screenshot bereit gestellt (webserver ist wo/welcher webserver?)
- Wer/wie wird link zum clipboard für aktuell aufgenommenen screenshot generiert
    - clipboard programm "verschiebt" screenshot mit generierter url

#### Projection
- eigener screenshotter/desktop capture service
