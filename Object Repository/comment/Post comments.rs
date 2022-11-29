<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post comments</name>
   <tag></tag>
   <elementGuidId>6de594db-80f2-4328-a584-7bf18d9a0c1e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{ \&quot;data\&quot; : \n [\n   {\n    \&quot;postId\&quot;: 1,\n    \&quot;name\&quot;: \&quot;salsa putri\&quot;,\n    \&quot;email\&quot;: \&quot;salsa@mailsac.com\&quot;,\n    \&quot;body\&quot;: \&quot;ini body dan tulisan\&quot;\n\t},\n   {\n    \&quot;postId\&quot;: 1,\n    \&quot;name\&quot;: \&quot;salsa putrii\&quot;,\n    \&quot;email\&quot;: \&quot;salsaa@mailsac.com\&quot;,\n    \&quot;body\&quot;: \&quot;inii body dan tulisan\&quot;\n\t}\n  ]\n}\n   &quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f43b008e-4de3-4eaf-845a-5816ed07dd62</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/comments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

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

//verifikasi data yang telah di input
//WS.verifyElementPropertyValue(response, 'postId', '1')
//WS.verifyElementPropertyValue(response, 'name', 'salsa putri')
//WS.verifyElementPropertyValue(response, 'email', 'salsa@mailsac.com')
//WS.verifyElementPropertyValue(response, 'body', 'ini body dan tulisan')
//WS.verifyElementPropertyValue(response, 'id', '501')

//verifikasi dua data
WS.verifyElementPropertyValue(response, 'data[0].postId', '1')
WS.verifyElementPropertyValue(response, 'data[0].name', 'salsa putri')
WS.verifyElementPropertyValue(response, 'data[0].email', 'salsa@mailsac.com')
WS.verifyElementPropertyValue(response, 'data[0].body', 'ini body dan tulisan')
WS.verifyElementPropertyValue(response, 'data[1].postId', '1')
WS.verifyElementPropertyValue(response, 'data[1].name', 'salsa putrii')
WS.verifyElementPropertyValue(response, 'data[1].email', 'salsaa@mailsac.com')
WS.verifyElementPropertyValue(response, 'data[1].body', 'inii body dan tulisan')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
