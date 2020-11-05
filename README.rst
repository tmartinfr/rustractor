
Rustractor
==========

Retrieves conversations from Slack to terminal.

Get Slack token
---------------
Create new app on https://api.slack.com/apps, with name `Rustractor`, and choose the workspace.

Get client ID & secret from your app page, store them in `CLIENT_ID` and `CLIENT_SECRET` environment variables.

Add `https://postman-echo.com/get` to the redirection URLs in the `OAuth &
Permissions` app page (or use an other HTTP target to access the code send by
Slack to the redirection URL).

Load this URL in a browser after adding your client ID at the end : ::

   https://slack.com/oauth/v2/authorize?user_scope=channels:read,channels:history,groups:read,groups:history,im:read,im:history,mpim:read,mpim:history,users:read&redirect_uri=https%3A%2F%2Fpostman-echo.com%2Fget&client_id=

Store the returned `code` in the CLIENT_CODE environment variable.

Get Oauth token in the next 10 minutes : ::

   curl -F code=$CLIENT_CODE -F client_id=$CLIENT_ID -F client_secret=$CLIENT_SECRET https://slack.com/api/oauth.v2.access

Store the returned token to the CLIENT_TOKEN environment variable, and backup it.

Example retrieving the channel list : ::

   curl -H "Authorization: Bearer $CLIENT_TOKEN" https://slack.com/api/conversations.list

