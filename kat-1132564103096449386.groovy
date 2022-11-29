import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//verifikasi status code 201

WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

////verifikasi data yang telah di create
//WS.verifyElementPropertyValue(response, 'title', 'ini title update')
//WS.verifyElementPropertyValue(response, 'body', 'ini body update')
//WS.verifyElementPropertyValue(response, 'userId', '1')
//WS.verifyElementPropertyValue(response, 'id', '101')


//verfikasi data yang telah di create
WS.verifyElementPropertyValue(response, '0.title', 'ini title update')
WS.verifyElementPropertyValue(response, '0.body', 'ini body update')
WS.verifyElementPropertyValue(response, '0.userId', '1')
WS.verifyElementPropertyValue(response, '1.title', 'medita')
WS.verifyElementPropertyValue(response, '1.body', 'kurnia')
WS.verifyElementPropertyValue(response, '1.userId', '1')
//WS.verifyElementPropertyValue(response, 'id', '101')