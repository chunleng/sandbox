local claims = std.extVar('claims');
local session = std.extVar('session');

{
  "claims": claims {
    "data": {
        "email": session.identity.traits.email
    }
  }
}
