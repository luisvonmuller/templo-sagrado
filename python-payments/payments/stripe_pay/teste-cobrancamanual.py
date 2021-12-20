# Stripe stuff
import stripe
stripe.api_key = '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'
try:
    charge = stripe.Charge.create(
        amount="1000",
        currency="eur",
        source="src_1IeQhkITn0AU8XFqwriBdlqU",
    )
    print('Consegui requisitar o charge')
except:
    print("An exception occurred when creating a charge")