#####
# Paypal client (SandBox or Live)
#from paypal_pay.paypal_sandbox_config import PayPalClient
from paypal_pay.paypal_live_config import PayPalClient
#####
from paypalcheckoutsdk.orders import OrdersCreateRequest, OrdersCaptureRequest

class CreateOrder(PayPalClient):

  #2. Set up your server to receive a call from the client
  """ This is the sample function to create an order. It uses the
    JSON body returned by buildRequestBody() to create an order."""

  def create_order(self, params, debug=False):
    request = OrdersCreateRequest()
    request.prefer('return=representation')
    #3. Call PayPal to set up a transaction
    request.request_body(params)
    response = self.client.execute(request)
    if debug:
      print('Status Code: ', response.status_code)
      print('Status: ', response.result.status)
      print('Order ID: ', response.result.id)
      print('Intent: ', response.result.intent)
      print('Links:')
      for link in response.result.links:
        print('\t{}: {}\tCall Type: {}'.format(link.rel, link.href, link.method))
      print('Total Amount: {} {}'.format(response.result.purchase_units[0].amount.currency_code,
                         response.result.purchase_units[0].amount.value))

    return response



##############################################################################
##############################################################################
##############################################################################
##############################################################################
##############################################################################
##############################################################################

class CaptureOrder(PayPalClient):

  #2. Set up your server to receive a call from the client
  """this sample function performs payment capture on the order.
  Approved order ID should be passed as an argument to this function"""

  def capture_order(self, order_id, debug=False):
    """Method to capture order using order_id"""
    request = OrdersCaptureRequest(order_id)
    #3. Call PayPal to capture an order
    response = self.client.execute(request)
    #4. Save the capture ID to your database. Implement logic to save capture to your database for future reference.
    if debug:
      print('Status Code: ', response.status_code)
      print('Status: ', response.result.status)
      print('Order ID: ', response.result.id)
      print('Links: ')
      for link in response.result.links:
        print('\t{}: {}\tCall Type: {}'.format(link.rel, link.href, link.method))
      print('Capture Ids: ')
      for purchase_unit in response.result.purchase_units:
        for capture in purchase_unit.payments.captures:
          print('\t', capture.id)
      print("Buyer:")
      print("\tEmail Address: {}\n\tName: {}\n\tPhone Number: {}".format(response.result.payer.email_address,
        response.result.payer.name.given_name + " " + response.result.payer.name.surname,
        response.result.payer.phone.phone_number.national_number))
    return response



params = {
  "intent": "CAPTURE",
  "application_context": {
    "brand_name": "Templo Sagrado",
    "landing_page": "BILLING",
    "shipping_preference": "NO_SHIPPING",
    # PAY_NOW or CONTINUE
    "user_action": "PAY_NOW"
  },
  "purchase_units": [
    {
      #"reference_id": "PUHF",
      #Product/item description
      "description": "Sporting Goods",
      "amount": {
        #The three-character ISO-4217 currency code that identifies the currency.
        "currency_code": "BRL",
        "value": "10.00",
      }
      
    }
  ]
}

#CreateOrder().create_order(params, debug=True)

#order_id = '6WB09360H54312823'
#CaptureOrder().capture_order(order_id, debug=True)