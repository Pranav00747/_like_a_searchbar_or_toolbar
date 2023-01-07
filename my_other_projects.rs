// Actually the mailbot project is almost ready but the problem is you have two off 2 way verfication because google have high level of security 
// because of hackers hacking emails the problem will get solved soon

fn get_conn()
{
  let prop = Properties();
  let session : Session = getSession(prop);
  let st : Store = session.getStore("imap");
  st.connect("imap.googlemail.com", "user_@gmail.com", "pass"); // Issue
  return st;
}

fn get_folder_by_name(name)
{
     let s = get_conn();
     let f : <Folder> = s.get_folder(name); // [Gmail/All Mails] or Spams or Trash or Sent or Drafts/Inbox kinda
     f.getMessges(); // Message arrays 
     let m : u32 = f.getMessageCounts(); // No of message using loop you acess each apart like from, rcpt_to, subject, body, attachments
 }
 // Deep Learning Kit i will upload soon.
       
 
