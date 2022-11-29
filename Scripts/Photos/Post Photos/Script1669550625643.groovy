import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

response = WS.sendRequest(findTestObject('photo/POST photos'))

//verifikasi status code
WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

////verifikasi data yang diinput
//WS.verifyElementPropertyValue(response, 'albumId', '1')
//WS.verifyElementPropertyValue(response, 'title', 'ini photo')
//WS.verifyElementPropertyValue(response, 'url', 'https://google.com')
//WS.verifyElementPropertyValue(response, 'thumbnailUrl', 'https://google.com')
//WS.verifyElementPropertyValue(response, 'id', '5001')

//verifikasi data yang diinput

WS.verifyElementPropertyValue(response, 'data[0].albumId', '1')
WS.verifyElementPropertyValue(response, 'data[0].title', 'ini photo')
WS.verifyElementPropertyValue(response, 'data[0].url', 'https://google.com')
WS.verifyElementPropertyValue(response, 'data[0].thumbnailUrl', 'https://google.com')
WS.verifyElementPropertyValue(response, 'data[1].albumId', '1')
WS.verifyElementPropertyValue(response, 'data[1].title', 'ini photoo')
WS.verifyElementPropertyValue(response, 'data[1].url', 'https://googlee.com')
WS.verifyElementPropertyValue(response, 'data[1].thumbnailUrl', 'https://googlee.com')