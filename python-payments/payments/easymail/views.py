from django.shortcuts import render
from django.http import HttpResponse, JsonResponse

import json

# Disable CSRF check for view
from django.views.decorators.csrf import csrf_exempt

# Email stuff
import smtplib
from email.mime.text import MIMEText

# EMAIL PARAMS
HOST_DOMAIN = 'easymail.easywebusa.com'
HOST_PORT = 587
USERNAME = 'atendimento@templo-sagrado.com'
PASSWORD = '2Y85X5k8Q3'


# RESPONSES
HEADER = {'Content-type': 'application/json', 'Accept': 'text/plain'}


@csrf_exempt
def send_email(request):
    print('Received a email request!')
    if request.method == 'POST':
        # read and load the data from post (Expecting JSON)
        try:
            mail_data = json.loads(request.read())
            print('Parseei o JSON')
            # FILLING MESSAGE
            try:
                print(mail_data)
                to_addrs = mail_data['to']
                # BODY
                message = MIMEText(mail_data['message'],'html')
                # SUBJECT
                message['subject'] = mail_data['subject']
                # FROM
                message['from'] = USERNAME
                # TO
                message['to'] = ', '.join(to_addrs)
                
                print('Peguei os campos da mensagem')
                try:
                    # CONECT TO SERVER WITH TLS
                    server = smtplib.SMTP(HOST_DOMAIN, HOST_PORT)
                    server.starttls()
                    # LOGIN INTO SERVER WITH CREDENTIALS
                    server.login(USERNAME, PASSWORD)
                    
                    try:
                        # SEND AN EMAIL BY SMTP
                        server.sendmail(USERNAME, to_addrs, message.as_string())
                        server.quit()
                        resp = JsonResponse({'success':True})
                        resp['Access-Control-Allow-Origin'] = '*'
                        return resp
                    except:
                        errorMsg = 'Some error on sending mail!'
                        resp = JsonResponse({
                            'success':False,
                            'errorMsg':errorMsg
                        })
                        resp['Access-Control-Allow-Origin'] = '*'
                        return resp
                except:
                    errorMsg = 'Some error on conecting to mail server!'
                    resp = JsonResponse({
                        'success':False,
                        'errorMsg':errorMsg
                    })
                    resp['Access-Control-Allow-Origin'] = '*'
                    return resp
            except:
                errorMsg = 'Some error on parsing JSON data!'
                resp = JsonResponse({
                    'success':False,
                    'errorMsg':errorMsg
                })
                resp['Access-Control-Allow-Origin'] = '*'
                return resp
        except:
            errorMsg = 'Expecting a JSON format!'
            resp = JsonResponse({
                'success':False,
                'errorMsg':errorMsg
            })
            resp['Access-Control-Allow-Origin'] = '*'
            return resp
    
    